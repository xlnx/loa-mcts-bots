import { Cell } from "./grid";
import { UI } from "./uiglobal";
import { Config } from "./global";
import { PieceType } from "./piece";
import { Timer } from "./timer";
import { Bot } from "./bot";

let hovered: Cell | undefined
let current: Cell | undefined
let time: number = 0
let gtime: number[] = [0, 0]
let turn: number = PieceType.Black
let timer!: Timer
let started: boolean = false

let pc: boolean[] = new Array(2)
let bot: Bot[] = new Array(2)
let accTime: number = 0
let id: number = 0

export class GameLogic {

	static init() {

		const bots = Bot.list()

		for (const player of UI.Menu.player) {

			player.addOptions("Player", ...bots)

		}

	}

	static start() {

		started = false

		this.cancelTimer()

		console.clear()

		current = undefined
		time = Config.Step
		gtime = [0, 0]
		turn = PieceType.Black
		started = true
		id += 1

		for (let i of [0, 1]) {

			const player = UI.Menu.player[i].getValue()

			if (pc[i] = player != `Player`) {
				bot[i] = new (Bot.get(player)!)(i)
			}

		}

		this.hoverCell(undefined)

		if (!UI.Grid.isReady()) {

			const thisId = id

			setTimeout(() => {

				if (id == thisId) {

					this.setTimer()

					this.checkBotMove()
				}

			}, Config.MoveAnimTime)

		} else {

			this.setTimer()

			this.checkBotMove()

		}

		UI.Grid.init()
		UI.Stat.turn.text = turn == PieceType.Black ? "黑" : "白"

	}

	static cancelTimer() {

		if (timer) {
			timer.cancel()
		}

	}

	static setTimer() {

		this.cancelTimer()

		timer = new Timer(() => {
			this.updateTime()
		}, 1000)

	}

	static playerWins(turn: number) {

		timer.cancel()
		timer = undefined

	}

	static updateTime() {

		time -= 1
		UI.Stat.timer.text = "" + time

		let gt = 0
		if (accTime === undefined) {
			gt = gtime[turn]
		} else {
			gt = gtime[turn] +
				(new Date().getTime() - accTime) / 1000
		}
		UI.Stat.gtimer.text = "" + gt.toFixed(2)

		if (time == 0) {

			this.playerWins(1 - turn)

		}

	}

	static setCell(cell: Cell) {

		if (started && this.shouldMove()) {

			if (!current || !current.piece) {
				current = cell
				if (current.piece) {

					const moves = UI.Grid.calcMoves(current.col, current.row)

					if (moves.length) {

						current.piece.setDrawing("highlight")

					} else {
						current = undefined
					}

				} else {
					current = undefined
				}
			} else {

				current.piece.setDrawing("default")

				if (current.col != cell.col || current.row != cell.row) {
					this.makeMove(current.col, current.row,
						cell.col, cell.row)
				}

				current = undefined
			}

		}

	}

	static gameOver() {

		started = false

		this.cancelTimer()

		// console.log("")

	}

	static checkBotMove() {

		accTime = new Date().getTime()

		if (started && pc[turn]) {

			const thisId = id

			setTimeout(() => {

				if (thisId == id) {

					const board = new Array(64).fill(-1)

					// for (let k = 0; k < 2; ++k) {
					for (let i = 0; i < 8; ++i) {
						for (let j = 0; j < 8; ++j) {
							const p = UI.Grid.getCell(i, j).piece
							if (!!p) {
								board[i + j * 8] = p.type
							}
						}
					}

					bot[turn].makeMove(board).then(res => {

						if (thisId == id) {

							const { x0, y0, x1, y1 } = Object.assign({
								x0: -1, y0: -1, x1: -1, y1: -1
							}, res)
							this.makeMove(x0, y0, x1, y1)

						}

					})

				}

			}, 0)

		}

	}

	static shouldMove() {

		return !pc[turn]

	}

	static makeMove(x0: number, y0: number, x1: number, y1: number) {

		if (!started) return

		UI.Stat.board.text = ""

		if (UI.Grid.makeMove(x0, y0, x1, y1)) {

			console.log(`%c  %c (${x0}, ${y0}) => (${x1}, ${y1})`,
				`background: ${turn == PieceType.Black ? "#75a" : "#0ac"}`,
				`background: #fff; color: #000`
			)

			gtime[turn] += (new Date().getTime() - accTime) / 1000
			accTime = undefined

			const ty = [PieceType.Black, PieceType.White]
			const win = ty.map(type => UI.Grid.checkWin(type))
			const hasMove = ty.map(type => UI.Grid.hasMove(type))

			// console.log(win, hasMove)

			if (!win[0] && !win[1]) {

				if (!hasMove[0] && !hasMove[1]) {

					// prompt draw
					UI.Stat.board.text = "双方无棋可走，平局"

					this.gameOver()

				} else {

					time = Config.Step
					this.setTimer()

					if (hasMove[1 - turn]) {

						turn = 1 - turn

					} else {

						// prompt nomove
						UI.Stat.board.text = `${turn == PieceType.Black ? "白" : "黑"}方无棋可走`

					}

					UI.Stat.turn.text = turn == PieceType.Black ? "黑" : "白"

				}

			} else {

				if (win[0] && win[1]) {

					UI.Stat.board.text = `双方胜利`

				} else {

					UI.Stat.board.text = `${win[PieceType.Black] ? "黑" : "白"}方胜利`

				}

				for (const t of ty) {
					if (win[t]) {
						UI.Grid.forEachPiece(t, p => {
							if (p.cell.col != x1 || p.cell.row != y1) {
								p.actions.delay(Config.MoveAnimTime)
							}
							p.actions.rotateBy(Math.PI, 300)
						})
					}
				}

				this.gameOver()

			}

			const thisId = id

			setTimeout(() => {

				if (thisId == id) {
					this.checkBotMove()
				}

			}, Config.MoveAnimTime)

		} else {

			console.log(`%c  %c (${x0}, ${y0}) => (${x1}, ${y1})`,
				`background: ${turn == PieceType.Black ? "#75a" : "#0ac"}`,
				`background: #fff; color: #f00`
			)

			if (pc[turn]) {

				UI.Stat.board.text = `非法棋步，${turn == PieceType.White ? "黑" : "白"}方胜利`

				this.gameOver()

			} else {

				UI.Stat.board.text = `非法棋步`

			}

		}

		this.hoverCell(hovered)

	}

	static removeHightlightCandidate() {

		for (let i = 0; i < 8; ++i) {

			for (let j = 0; j < 8; ++j) {

				const c = UI.Grid.getCell(i, j)
				c.setDrawing("default")

				if (c.piece) {
					c.piece.setDrawing("default")
				}

			}
		}

	}

	static highlightCandidate(cell: Cell) {

		this.removeHightlightCandidate()

		if (cell.piece) {

			const moves = UI.Grid.calcMoves(cell.col, cell.row)
			for (const { x, y } of moves) {
				UI.Grid.getCell(x, y).setDrawing("highlight")
			}

			cell.piece.setDrawing("highlight")

		}

	}

	static hoverCell(cell: Cell) {

		if (hovered = cell) {

			if (!current || !current.piece) {

				if (started && !!cell.piece && cell.piece.type == turn && this.shouldMove()) {

					this.highlightCandidate(cell)

				} else {

					this.removeHightlightCandidate()

				}

			}

		} else {

			this.removeHightlightCandidate()

		}

	}

	static unhoverCell(cell: Cell) {

		if (hovered == cell) hovered = undefined

	}

}

