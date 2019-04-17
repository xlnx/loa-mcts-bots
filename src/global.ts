import { Vector, Texture, Engine, Color, DisplayMode } from "excalibur";

export const Config = {
	Cell: {
		Width: 45,
		Height: 45
	},
	Piece: {
		Width: 36,
		Height: 36
	},
	MenuButton: {
		Width: 100,
		Height: 48
	},
	Step: 60,
	MoveAnimTime: 350
}

export const Resources = {
	BackgroundTexture: new Texture("images/bg.png"),
	LogoTexture: new Texture("images/logo.png"),
	Tile0Texture: new Texture("images/Tile0.png"),
	Tile1Texture: new Texture("images/Tile1.png"),
	BoardTexture: new Texture("images/board.png"),
	StandardBtnTexture: new Texture("images/standard.png")
}

export const Palette = {
	PieceColor: [
		Color.fromHex("#00718D"),
		Color.fromHex("#7A5CA7")
	]
}

export const Game = new Engine({
	width: 0,
	height: 0,
	backgroundColor: new Color(0, 0, 0),
	displayMode: DisplayMode.FullScreen,
	suppressPlayButton: true
})
