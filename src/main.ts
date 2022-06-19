import './style.css';
const rust = import('../reversi-agent/pkg');

const startButton = document.querySelector('#start-button') as HTMLButtonElement;
const endButton = document.querySelector('#end-button') as HTMLButtonElement;
const undoButton = document.querySelector('#undo-button') as HTMLButtonElement;

const firstPlayerSelect = document.querySelector('#first-player-select') as HTMLSelectElement;
const secondPlayerSelect = document.querySelector('#second-player-select') as HTMLSelectElement;

const chessboardTable = document.querySelector('#chessboard-table') as HTMLTableElement;
const chessboardClickCallbacks: ((row: number, col: number) => boolean)[] = [];

function initChessboardTable() {
  function handleClick(row: number, col: number): void {
    for (let i = 0; i < chessboardClickCallbacks.length;) {
      if (chessboardClickCallbacks[i](row, col)) {
        chessboardClickCallbacks.splice(i, 1);
      } else {
        ++i;
      }
    }
  }

  chessboardTable.innerHTML = '';
  for (let row = 0; row < 8; ++row) {
    const tableRow = document.createElement('tr');
    for (let col = 0; col < 8; ++col) {
      const disc = document.createElement('div');
      disc.className = 'disc invisible candidate';
      const tableCell = document.createElement('td');
      tableCell.addEventListener('click', () => handleClick(row, col));
      tableCell.appendChild(disc);
      tableRow.appendChild(tableCell);
    }
    chessboardTable.appendChild(tableRow);
  }
}

initChessboardTable();

enum Disc {
  None = 0,
  First = 1,
  Second = -1,
}

type GameState = {
  player: Disc.First | Disc.Second,
  chessboardData: Disc[][],
  legalMoves: boolean[][],
};

async function updateChessboardTable(
  { player, chessboardData, legalMoves }: GameState): Promise<void> {
  for (let row = 0; row < 8; ++row) {
    for (let col = 0; col < 8; ++col) {
      const disc = chessboardTable.rows[row].cells[col].querySelector('.disc') as HTMLDivElement;
      switch (chessboardData[row][col]) {
        case Disc.First:
          disc.className = 'disc first';
          break;
        case Disc.Second:
          disc.className = 'disc second';
          break;
        case Disc.None:
          if (!legalMoves[row][col]) {
            disc.className = 'disc invisible candidate';
          } else {
            switch (player) {
              case Disc.First:
                disc.className = 'disc candidate first';
                break;
              case Disc.Second:
                disc.className = 'disc candidate second';
            }
          }
      }
    }
  }
  await new Promise(resolve => setTimeout(resolve, 800));
}

function getLegalMoves(player: Disc.First | Disc.Second, chessboardData: Disc[][]): boolean[][] {
  function isLegalMoveInDirection(
    row: number, col: number, rowStep: number, colStep: number): boolean {
    let hasScore = false;
    while (row += rowStep, col += colStep, (0 <= row && row < 8) && (0 <= col && col < 8)) {
      switch (chessboardData[row][col]) {
        case +player:
          return hasScore;
        case -player:
          hasScore = true;
          break;
        default:
          return false;
      }
    }
    return false;
  }

  function isLegalMove(row: number, col: number): boolean {
    for (let rowStep = -1; rowStep <= 1; ++rowStep) {
      for (let colStep = -1; colStep <= 1; ++colStep) {
        if (!(rowStep === 0 && colStep === 0) &&
          isLegalMoveInDirection(row, col, rowStep, colStep)) {
          return true;
        }
      }
    }
    return false;
  }

  const legalMoves = new Array(8);
  for (let row = 0; row < 8; ++row) {
    const legalMovesRow = new Array(8);
    for (let col = 0; col < 8; ++col) {
      legalMovesRow[col] = chessboardData[row][col] === Disc.None && isLegalMove(row, col);
    }
    legalMoves[row] = legalMovesRow;
  }
  return legalMoves;
}

function hasLegalMove(legalMoves: boolean[][]) {
  for (const legalMovesRow of legalMoves) {
    for (const isLegalMove of legalMovesRow) {
      if (isLegalMove) {
        return true;
      }
    }
  }
  return false;
}

function cloneChessboardData(chessboardData: Disc[][]): Disc[][] {
  const clonedData = new Array(8);
  for (let row = 0; row < 8; ++row) {
    clonedData[row] = [...chessboardData[row]];
  }
  return clonedData;
}

function makeMove(
  player: Disc.First | Disc.Second, chessboardData: Disc[][], move: [number, number]): void {
  function flipDiscsInDirection(rowStep: number, colStep: number): void {
    let [row, col] = move;
    do {
      row += rowStep;
      col += colStep;
      if (!(0 <= row && row < 8) || !(0 <= col && col < 8) ||
        chessboardData[row][col] === Disc.None) {
        return;
      }
    } while (chessboardData[row][col] !== player);
    while (row -= rowStep, col -= colStep, !(row === move[0] && col === move[1])) {
      chessboardData[row][col] = -chessboardData[row][col];
    }
  }

  chessboardData[move[0]][move[1]] = player;
  for (let rowStep = -1; rowStep <= 1; ++rowStep) {
    for (let colStep = -1; colStep <= 1; ++colStep) {
      if (!(rowStep === 0 && colStep === 0)) {
        flipDiscsInDirection(rowStep, colStep);
      }
    }
  }
}

type Player = {
  undoable: boolean,
  play: (state: GameState) => Promise<[number, number]>,
};

class Game {
  firstPlayer: Player;
  secondPlayer: Player;
  endFlag: boolean;
  undoFlag: boolean;
  state: GameState;
  stateHistory: GameState[];

  constructor(firstPlayer: Player, secondPlayer: Player) {
    startButton.disabled = true;
    firstPlayerSelect.disabled = true;
    secondPlayerSelect.disabled = true;

    this.firstPlayer = firstPlayer;
    this.secondPlayer = secondPlayer;
    this.endFlag = false;
    this.undoFlag = false;
    this.stateHistory = [];

    const chessboardData = new Array(8);
    for (let row = 0; row < 8; ++row) {
      chessboardData[row] = new Array(8).fill(Disc.None);
    }
    const player = Disc.First;
    chessboardData[3][4] = Disc.First;
    chessboardData[4][3] = Disc.First;
    chessboardData[3][3] = Disc.Second;
    chessboardData[4][4] = Disc.Second;
    this.state = {
      player,
      chessboardData,
      legalMoves: getLegalMoves(player, chessboardData),
    };

    endButton.disabled = false;
  }

  waitForEndOrUndoClick(): Promise<null> {
    this.endFlag = false;
    this.undoFlag = false;
    const self = this;
    return new Promise(resolve => {
      endButton.onclick = () => {
        endButton.onclick = () => { };
        undoButton.onclick = () => { };
        self.endFlag = true;
        resolve(null);
      };
      undoButton.onclick = () => {
        endButton.onclick = () => { };
        undoButton.onclick = () => { };
        self.undoFlag = true;
        resolve(null);
      };
    });
  }

  async undo(): Promise<void> {
    if (this.stateHistory.length === 1) {
      undoButton.disabled = true;
    }
    this.state = this.stateHistory.pop();
    await updateChessboardTable(this.state);
  }

  async end(): Promise<void> {
    endButton.disabled = true;
    undoButton.disabled = true;

    const emptyChessboardData = new Array(8);
    const emptyLegalMoves = new Array(8);
    for (let row = 0; row < 8; ++row) {
      emptyChessboardData[row] = new Array(8).fill(Disc.None);
      emptyLegalMoves[row] = new Array(8).fill(false);
    }
    await updateChessboardTable({
      player: Disc.First,
      chessboardData: emptyChessboardData,
      legalMoves: emptyLegalMoves,
    });

    startButton.disabled = false;
    firstPlayerSelect.disabled = false;
    secondPlayerSelect.disabled = false;
  }

  async run(): Promise<void> {
    await Promise.race([updateChessboardTable(this.state), this.waitForEndOrUndoClick()]);
    if (this.endFlag) {
      await this.end();
      return;
    }

    while (true) {
      if (!hasLegalMove(this.state.legalMoves)) {
        const nextLegalMoves = getLegalMoves(-this.state.player, this.state.chessboardData);
        if (!hasLegalMove(nextLegalMoves)) {
          break;
        }
        alert(`${this.state.player === Disc.First ? 'Black' : 'White'} is out of move.`);
        this.state = {
          player: -this.state.player,
          chessboardData: this.state.chessboardData,
          legalMoves: nextLegalMoves,
        };
      } else {
        const player = this.state.player === Disc.First ? this.firstPlayer : this.secondPlayer;
        const move = await Promise.race([player.play(this.state), this.waitForEndOrUndoClick()]);
        if (this.endFlag) {
          await this.end();
          return;
        }
        if (this.undoFlag) {
          await this.undo();
          continue;
        }
        if (player.undoable) {
          this.stateHistory.push(this.state);
          undoButton.disabled = false;
        }
        const nextChessboardData = cloneChessboardData(this.state.chessboardData);
        makeMove(this.state.player, nextChessboardData, move);
        this.state = {
          player: -this.state.player,
          chessboardData: nextChessboardData,
          legalMoves: getLegalMoves(-this.state.player, nextChessboardData),
        };
      }

      await Promise.race([updateChessboardTable(this.state), this.waitForEndOrUndoClick()]);
      if (this.endFlag) {
        await this.end();
        return;
      }
      if (this.undoFlag) {
        await this.undo();
        continue;
      }
    }

    let scoreSum = 0;
    for (const scoreRow of this.state.chessboardData) {
      for (const score of scoreRow) {
        scoreSum += score;
      }
    }
    let winnerMessage = 'Game over! ';
    if (scoreSum > 0) {
      winnerMessage += 'Black is the winner.';
    } else if (scoreSum < 0) {
      winnerMessage += 'White is the winner.';
    } else {
      winnerMessage += 'The game is a tie.';
    }
    alert(winnerMessage);

    undoButton.disabled = true;
    while (true) {
      await this.waitForEndOrUndoClick();
      if (this.endFlag) {
        await this.end();
        return;
      }
    }
  }
}

function humanPlayerPlay(
  { player, chessboardData, legalMoves }: GameState): Promise<[number, number]> {
  return new Promise(resolve => {
    chessboardClickCallbacks.push((row, col) => {
      if (legalMoves[row][col]) {
        resolve([row, col]);
        return true;
      } else {
        return false;
      }
    });
  });
}

async function cpuPlayerPlay(
  { player, chessboardData, legalMoves }: GameState,
  maxDepth: number,
  useCoinParity: boolean,
  useActualMobility: boolean,
  usePotentialMobility: boolean,
  useCornerScore: boolean,
  useStabilityScore: boolean,
): Promise<[number, number]> {
  const flatChessboard = new Int8Array(64);
  for (let i = 0, row = 0; row < 8; ++row) {
    for (let col = 0; col < 8; ++col, ++i) {
      flatChessboard[i] = chessboardData[row][col];
    }
  }
  const flatMove = (await rust).play(
    player,
    flatChessboard,
    maxDepth,
    useCoinParity,
    useActualMobility,
    usePotentialMobility,
    useCornerScore,
    useStabilityScore,
  );
  const moveCol = flatMove % 8, moveRow = Math.round((flatMove - moveCol) / 8);
  return [moveRow, moveCol];
}

function createPlayer(option: string): Player {
  switch (option) {
    case 'cpu-easy':
      return {
        undoable: false,
        play: state => cpuPlayerPlay(
          state,
          4,     // maxDepth
          true,  // useCoinParity
          false, // useActualMobility
          false, // usePotentialMobility
          false, // useCornerScore
          true,  // useStabilityScore
        ),
      };
    case 'cpu-normal':
      return {
        undoable: false,
        play: state => cpuPlayerPlay(
          state,
          6,     // maxDepth
          true,  // useCoinParity
          false, // useActualMobility
          false, // usePotentialMobility
          true,  // useCornerScore
          true,  // useStabilityScore
        ),
      };
    case 'cpu-hard':
      return {
        undoable: false,
        play: state => cpuPlayerPlay(
          state,
          8,     // maxDepth
          true,  // useCoinParity
          true,  // useActualMobility
          true,  // usePotentialMobility
          true,  // useCornerScore
          true,  // useStabilityScore
        ),
      };
    default:
      return {
        undoable: true,
        play: humanPlayerPlay,
      };
  }
}

startButton.onclick = async () => {
  const firstPlayer = createPlayer(firstPlayerSelect.value);
  const secondPlayer = createPlayer(secondPlayerSelect.value);
  await new Game(firstPlayer, secondPlayer).run();
};
