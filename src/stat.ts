import { Actor, Label, TextAlign, Color } from "excalibur";
import { Game } from "./global";

export class Stat extends Actor {

	public readonly turn!: Label
	public readonly timer!: Label
	public readonly gtimer!: Label
	public readonly board!: Label

	constructor() {

		super()

		this.board = new Label("", 0, -190, "museo-sans, arial")
		this.board.color = new Color(255, 255, 255)
		this.board.fontSize = 20
		this.board.opacity = 0.7
		this.board.textAlign = TextAlign.Center
		Game.add(this.board)

		this.turn = new Label("黑", -100, -240, "museo-sans, arial")
		this.turn.color = new Color(255, 255, 255)
		this.turn.fontSize = 40
		this.turn.opacity = 0.7
		this.turn.textAlign = TextAlign.Center
		Game.add(this.turn)

		this.timer = new Label("0", 0, -240, "museo-sans, arial")
		this.timer.color = new Color(255, 255, 255)
		this.timer.fontSize = 40
		this.timer.opacity = 0.7
		this.timer.textAlign = TextAlign.Center
		Game.add(this.timer)

		this.gtimer = new Label("0", 100, -240, "museo-sans, arial")
		this.gtimer.color = new Color(255, 255, 255)
		this.gtimer.fontSize = 40
		this.gtimer.opacity = 0.7
		this.gtimer.textAlign = TextAlign.Center
		Game.add(this.gtimer)

		const turn = new Label("当前", -100, -220, "museo-sans, arial")
		turn.color = new Color(255, 255, 255)
		turn.fontSize = 20
		turn.opacity = 0.2
		turn.textAlign = TextAlign.Center
		Game.add(turn)

		const timer = new Label("步时", 0, -220, "museo-sans, arial")
		timer.color = new Color(255, 255, 255)
		timer.fontSize = 20
		timer.opacity = 0.2
		timer.textAlign = TextAlign.Center
		Game.add(timer)

		const gtimer = new Label("局时", 100, -220, "museo-sans, arial")
		gtimer.color = new Color(255, 255, 255)
		gtimer.fontSize = 20
		gtimer.opacity = 0.2
		gtimer.textAlign = TextAlign.Center
		Game.add(gtimer)

	}

	onInitialize() {

		this.z = 200

	}

	update() {

	}

}
