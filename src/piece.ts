import { Color, Engine, Sprite, Actor } from "excalibur";
import { Config, Resources, Game, Palette } from "./global";
import { Cell } from "./grid";

export enum PieceType {
	Black = 0,
	White = 1
}

const PieceTypes = [PieceType.Black, PieceType.White]
const PieceTypeToColor = Palette.PieceColor
const PieceTypeToTexture = [Resources.Tile0Texture, Resources.Tile1Texture]
const PieceTypeToSprites = PieceTypeToTexture.map(tex =>
	Array.from(new Array(2).keys()).map(i =>
		new Sprite(tex, Config.Piece.Width * i, 0,
			Config.Piece.Width, Config.Piece.Height)))

export class Piece extends Actor {

	private static id: number = 0

	public cell?: Cell

	public readonly id: number = Piece.id++

	constructor(public readonly type: PieceType) {

		super(0, 0, Config.Piece.Width, Config.Piece.Height, PieceTypeToColor[type])

		this.addDrawing("default", PieceTypeToSprites[this.type][0])
		this.addDrawing("highlight", PieceTypeToSprites[this.type][1])

		this.setDrawing("default")

	}

	update(engine: Engine, delta: number) {

		super.update(engine, delta)

	}

	// draw(ctx: CanvasRenderingContext2D) {

	// 	this.debugDraw(ctx)
	// }

}
