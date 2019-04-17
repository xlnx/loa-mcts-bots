import { Actor, Engine, Vector, EasingFunctions, GameEvent, Color, Label, TextAlign } from "excalibur";
import { Resources, Config, Game } from "../glob/global";
import { UI } from "../glob/uiglobal";
import { GameLogic } from "../glob/gamelogic";
import { Dropdown } from "./dropdown";

export class MenuButton extends Actor {

	private label: Label

	constructor(text: string, action: (evt?: GameEvent<any>) => void, x: number, y: number) {

		super(x, y, Config.MenuButton.Width, Config.MenuButton.Height, new Color(131, 157, 99))

		const label = new Label(text, 0, 0, "museo-sans, arial")
		this.add(label)
		label.color = new Color(255, 255, 255)
		label.fontSize = 16
		label.opacity = 0.8
		label.textAlign = TextAlign.Center
		label.y = 10
		this.label = label

		this.opacity = 0.6

		// this.scale.setTo(Util.clamp(Config.Scale.x, 0, 1),
		// 	Util.clamp(Config.Scale.y, 0, 1))

		// this.anchor.setTo(.5, .7)

		// this.addDrawing(sprite)

		this.off("pointerup", action)
		this.on("pointerup", action)

	}

}

export class MainMenu extends Actor {

	private static LogoPos = new Vector(0, 300)
	private static StandardButtonPos = new Vector(42, 150 + Config.MenuButton.Height + 20)

	private logo!: Actor

	public readonly player: Dropdown[] = []

	constructor() {

		super()

		this.color = new Color(0, 0, 0)
		this.opacity = 0.6
		this.anchor.setTo(.5, 0)

		this.logo = new Actor(this.x, this.getTop() - MainMenu.LogoPos.y - 50)
		this.logo.anchor.setTo(.5, .5)
		this.logo.addDrawing(Resources.LogoTexture.asSprite())
		// this.logo.currentDrawing.scale.setTo(
		// 	0.7 * Config.Scale.x, 0.7 * Config.Scale.y)
		// this.logo.currentDrawing.

		// Game.add(this.logo)

		Game.add(new MenuButton("Start",
			() => GameLogic.start(),
			this.x, this.y + MainMenu.StandardButtonPos.y))

		this.logo.actions
			.easeTo(
				this.x, this.getTop() - MainMenu.LogoPos.y,
				650, EasingFunctions.EaseInOutQuad
			)

		this.player[0] = new Dropdown(
			this.x - 110, this.y + MainMenu.StandardButtonPos.y)
		this.player[1] = new Dropdown(
			this.x + 110, this.y + MainMenu.StandardButtonPos.y)

		Game.add(this.player[0])
		Game.add(this.player[1])

	}

	onInitialize(engine: Engine) {

		super.onInitialize(engine)

		this.z = -50

	}

	update(engine: Engine, delta: number) {

		super.update(engine, delta)

		// const { x, y } = Game.worldToScreenCoordinates(
		// 	new Vector(UI.Grid.x, UI.Grid.y))
		// this.x = x; this.y = y

		this.x = UI.Grid.getCenter().x
		this.y = UI.Grid.getTop() - 130

		this.setWidth(UI.Grid.getWidth())
		this.setHeight(UI.Grid.getHeight() + 230)

	}

}