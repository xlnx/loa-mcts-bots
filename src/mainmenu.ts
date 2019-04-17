import { Actor, Engine, Vector, EasingFunctions, Sprite, Util, GameEvent, Color, Label, TextAlign } from "excalibur";
import { Resources, Config, Game } from "./global";
import { UI } from "./uiglobal";
import { Action } from "excalibur/dist/Actions/Action";
import { GameLogic } from "./gamelogic";

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
		this.label = label

		this.opacity = 0.6

		// this.scale.setTo(Util.clamp(Config.Scale.x, 0, 1),
		// 	Util.clamp(Config.Scale.y, 0, 1))

		this.anchor.setTo(.5, .7)

		// this.addDrawing(sprite)

		this.off("pointerup", action)
		this.on("pointerup", action)

	}

	// draw(ctx: CanvasRenderingContext2D, delta: number) {

	// 	this.label.draw(ctx, delta)
	// 	this.debugDraw(ctx)
	// }

}

export class MainMenu extends Actor {

	private static LogoPos = new Vector(0, 300)
	private static StandardButtonPos = new Vector(42, 170 + Config.MenuButton.Height + 20)

	private logo!: Actor

	onInitialize(engine: Engine) {

		super.onInitialize(engine)

		this.color = new Color(0, 0, 0)
		this.opacity = 0.6
		this.z = -50
		this.anchor.setTo(.5, 0)

		this.logo = new Actor(this.x, this.getTop() - MainMenu.LogoPos.y - 50)
		this.logo.anchor.setTo(.5, .5)
		this.logo.addDrawing(Resources.LogoTexture.asSprite())
		// this.logo.currentDrawing.scale.setTo(
		// 	0.7 * Config.Scale.x, 0.7 * Config.Scale.y)
		// this.logo.currentDrawing.

		// Game.add(this.logo)

		Game.add(new MenuButton("Bot/Bot",
			() => GameLogic.start(true, true),
			this.x + 120, this.y + MainMenu.StandardButtonPos.y))

		Game.add(new MenuButton("Player/Bot",
			() => GameLogic.start(false, true),
			this.x, this.y + MainMenu.StandardButtonPos.y))

		Game.add(new MenuButton("Player/Player",
			() => GameLogic.start(false, false),
			this.x - 120, this.y + MainMenu.StandardButtonPos.y))

		this.logo.actions
			.easeTo(
				this.x, this.getTop() - MainMenu.LogoPos.y,
				650, EasingFunctions.EaseInOutQuad
			)

		// this.button.actions
		// 	.easeTo(
		// 		this.x, this.y + MainMenu.StandardButtonPos.y,
		// 		650, EasingFunctions.EaseInOutQuad
		// 	)

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