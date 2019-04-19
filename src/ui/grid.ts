import { Config, Resources, Game } from "../glob/global";
import { Piece, PieceType } from "./piece";
import { Vector, Actor, Sprite, EasingFunctions } from "excalibur";
import { GameLogic } from "../glob/gamelogic";

const Dim = 8

class DisjointSet<T> {

	private readonly pre: Map<T, T>

	constructor(elems: T[]) {

		this.pre = new Map()

		for (const e of elems) {
			this.pre.set(e, undefined)
		}

	}

	join(a: T, b: T) {

		a = this.get(a)
		b = this.get(b)
		if (a != b) {
			this.pre.set(b, a)
		}

	}

	private get(a: T) {

		while (this.pre.get(a) != undefined) {
			a = this.pre.get(a)
		}
		return a

	}

	isJoint() {

		let root: T | undefined = undefined
		for (const e of this.pre.keys()) {
			const r = this.get(e)
			if (root === undefined) root = r
			else if (root != r) {
				return false
			}
		}

		return true

	}

}

export class Cell extends Actor {

	private _piece?: Piece

	constructor(
		public readonly col: number,
		public readonly row: number,
		// public readonly grid: Grid
	) {

		super(0, 0, Config.Cell.Width, Config.Cell.Height)

		const cw = Config.Cell.Width// * Config.Scale.x
		const ch = Config.Cell.Height// * Config.Scale.y
		const { x, y } = new Vector((this.col - Dim / 2) * cw + (cw / 2),
			(this.row - Dim / 2) * ch + (ch / 2))

		this.x = x; this.y = y

		const sprite = new Sprite(Resources.BoardTexture, 30, 30, 30, 30)
		sprite.scale.setTo(Config.Cell.Width / 30, Config.Cell.Height / 30)

		const sprite1 = sprite.clone()
		sprite.darken(0.8)
		sprite1.darken(0.5)

		this.opacity = 0.7

		this.addDrawing("default", sprite)
		this.addDrawing("highlight", sprite1)

		this.setDrawing("default")

		this.on("pointerdown", () => {

			GameLogic.setCell(this)

		})

		this.on("pointerenter", () => {

			GameLogic.hoverCell(this)

		})

		this.on("pointerleave", () => {

			GameLogic.unhoverCell(this)

		})

		// this.on("pointerdragstart", (evt: any) => {
		// 	const { x, y } = evt.coordinates.worldPos
		// 	if (this.piece) {
		// 		this.piece.x = x; this.piece.y = y
		// 	}
		// })

		// this.on("pointerdragend", (evt: any) => {
		// 	const { x, y } = evt.coordinates.worldPos
		// 	if (this.piece) {
		// 		this.piece.x = x; this.piece.y = y
		// 	}
		// })

		// this.on("pointerdragmove", (evt: any) => {
		// 	const { x, y } = evt.coordinates.worldPos
		// 	if (this.piece) {
		// 		// this.piece.actions.easeTo(x, y, 0)
		// 		this.piece.x = x; this.piece.y = y
		// 	}
		// })

	}

	set piece(other: Piece | undefined) {

		const { x, y } = this.getCenter()
		this._piece = other

		if (!!other) {
			other.actions
				.easeTo(
					x, y, Config.MoveAnimTime, EasingFunctions.EaseInOutQuad
				)
			other.cell = this
		}

	}

	get piece(): Piece | undefined {
		return this._piece
	}

}

export class Grid extends Actor {

	private readonly container = new Actor

	private readonly cells: Cell[] = new Array(Dim * Dim)

	private ready: boolean = false
	private turn = PieceType.Black

	private pieces: Piece[] = []

	constructor() {

		super(0, 0, Config.Cell.Width * Dim, Config.Cell.Height * Dim)

		for (let i = 0; i < Dim; ++i) {
			for (let j = 0; j < Dim; ++j) {

				const cell = new Cell(j, i)
				this.cells[j + i * Dim] = cell
				Game.add(cell)

			}
		}

		this.init()

	}

	onInitialize() {

		this.add(this.container)

	}

	isReady() {

		return this.ready

	}

	update() {

		this.container.x = this.getLeft()
		this.container.y = this.getTop()

	}

	getCell(x: number, y: number): Cell | undefined {

		if (x < 0 || x >= Dim) return
		if (y < 0 || y >= Dim) return

		return this.cells[x + y * Dim]

	}

	getRow(y: number): Cell[] {

		return Array.from(new Array(Dim).keys())
			.map(x => this.getCell(x, y))

	}

	getCol(x: number): Cell[] {

		return Array.from(new Array(Dim).keys())
			.map(y => this.getCell(x, y))

	}

	getLine(x0: number, y0: number, dx: number, dy: number): number {

		if (dx == dy && dy == 0) return 0

		if (dx != 0)
			return Array.from(new Array(Dim).keys())
				.map(x => {
					return {
						x, y: (x - x0) * dy / dx + y0
					}
				})
				.map(e => this.getCell(e.x, e.y))
				.filter(e => !!e && !!e.piece)
				.length
		else
			return Array.from(new Array(Dim).keys())
				.map(y => {
					return {
						y, x: (y - y0) * dx / dy + x0
					}
				})
				.map(e => this.getCell(e.x, e.y))
				.filter(e => !!e && !!e.piece)
				.length

	}

	setCell(x: number, y: number, piece?: Piece) {

		const cell = this.getCell(x, y)

		if (cell) {
			cell.piece = piece
		}

	}

	draw(ctx: CanvasRenderingContext2D, delta: number) {

		super.draw(ctx, delta)

		this.container.draw(ctx, delta)

	}

	checkValid(x0: number, y0: number, x1: number, y1: number) {

		const dx = x1 - x0
		const dy = y1 - y0

		const adx = Math.abs(dx)
		const ady = Math.abs(dy)

		const idx = Math.sign(dx)
		const idy = Math.sign(dy)

		if (
			x0 >= 0 && y0 >= 0 && x1 >= 0 && y1 >= 0 &&
			x0 < Dim && y0 < Dim && x1 < Dim && y1 < Dim
		) {

			if (
				(!!dx || !!dy) && ((!dx || !dy) || (adx == ady))
			) {
				if (
					this.getLine(x0, y0, idx, idy) == Math.max(adx, ady)
				) {
					const p = this.getCell(x0, y0).piece
					const q = this.getCell(x1, y1).piece

					if (
						(!!p && p.type == this.turn) &&
						(!q || q.type != this.turn)
					) {

						for (
							let x = x0 + idx, y = y0 + idy;
							x != x1 || y != y1;
							x += idx, y += idy
						) {

							const c = this.getCell(x, y)
							if (!!c.piece && c.piece.type != this.turn) {
								return false
							}
						}

						return true
					}
				}
			}
		}

		return false

	}

	calcMoves(x0: number, y0: number): { x: number, y: number }[] {

		const res: any[] = []

		const dxdy = [
			[1, 0], [-1, 0], [0, 1], [0, -1],
			[1, 1], [1, -1], [-1, -1], [-1, 1]
		]

		for (const [idx, idy] of dxdy) {

			const d = this.getLine(x0, y0, idx, idy)

			let x = x0 + d * idx, y = y0 + d * idy

			if (this.checkValid(x0, y0, x, y)) {
				res.push({ x, y })
			}

		}

		return res

	}

	callAllMoves(turn: number): { x0: number, y0: number, x1: number, y1: number }[] {

		const res: any[] = []

		this.forEachPiece(turn, (p: Piece) => {
			const { col: i, row: j } = p.cell
			res.push(...this.calcMoves(i, j)
				.map(e => {
					return { x0: i, y0: j, x1: e.x, y1: e.y }
				}))
		})

		return res

	}

	hasMove(turn: number) {

		return this.callAllMoves(turn).length != 0

	}

	makeMove(x0: number, y0: number, x1: number, y1: number): boolean {

		const c0 = this.getCell(x0, y0)
		const c1 = this.getCell(x1, y1)

		if (!!c0 && !!c1) {

			const p0 = c0.piece
			const p1 = c1.piece

			if (!!p0) {

				if (this.checkValid(x0, y0, x1, y1)) {

					this.ready = false

					c1.piece = p0
					c0.piece = undefined

					if (!!p1) {

						Game.remove(p1)

					}

					this.turn = 1 - this.turn

					return true

				}

			}

		}

		return false

	}

	checkWin(turn: number): boolean {

		const dxdy = [
			[1, 0], [-1, 0], [0, 1], [0, -1],
			[1, 1], [1, -1], [-1, -1], [-1, 1]
		]

		let ps: Piece[] = []
		this.forEachPiece(1 - turn, p => { ps.push(p) })
		if (ps.length == 1) return true

		ps = []
		this.forEachPiece(turn, p => { ps.push(p) })
		const set = new DisjointSet<Piece>(ps)
		for (const p of ps) {
			const { col: i, row: j } = p.cell
			for (const [idx, idy] of dxdy) {
				const c = this.getCell(i + idx, j + idy)
				if (!!c && !!c.piece && c.piece.type == turn) {
					set.join(p, c.piece)
				}
			}
		}

		return ps.length > 1 && set.isJoint()

	}

	forEachPiece(turn: number, fn: (p: Piece) => void) {

		for (let i = 0; i < Dim; ++i) {
			for (let j = 0; j < Dim; ++j) {
				const p = this.getCell(i, j).piece
				if (!!p && p.type == turn) {
					fn(p)
				}
			}
		}

	}

	init() {

		if (!this.ready) {

			for (const piece of this.pieces) {

				Game.remove(piece)

			}

			const w = PieceType.White
			const b = PieceType.Black
			const e: undefined = undefined

			const board = [
				e, b, b, b, b, b, b, e,
				w, e, e, e, e, e, e, w,
				w, e, e, e, e, e, e, w,
				w, e, e, e, e, e, e, w,
				w, e, e, e, e, e, e, w,
				w, e, e, e, e, e, e, w,
				w, e, e, e, e, e, e, w,
				e, b, b, b, b, b, b, e,
			]

			for (let i = 0; i != Dim; ++i) {
				for (let j = 0; j != Dim; ++j) {
					const type = board[j + i * Dim]
					if (type !== undefined) {

						const piece = new Piece(type)
						this.setCell(j, i, piece)
						this.pieces.push(piece)
						Game.add(piece)

					} else {
						this.setCell(j, i, undefined)
					}
				}
			}

			this.turn = PieceType.Black

			this.ready = true

		}
	}

}
