import { Actor, Vector, Texture, Engine } from "excalibur";
import { Game } from "./global";

export class Background extends Actor {

	private origin: Vector

	constructor(private readonly texture: Texture) {

		super(0, 0,
			Game.drawWidth + 5 * texture.width,
			texture.height * 2)

		this.anchor.setTo(0, 0)

		this.origin = new Vector(-Game.drawWidth - 2 * texture.width,
			- texture.height / 2)

		this.addDrawing(texture)

	}

	onInitialize() {

		this.z = -100

		const { x, y } = this.origin

		this.x = x; this.y = y

	}

	update(engine: Engine, delta: number) {

		const { x, y } = this.origin

		this.x = -Game.drawWidth - 2 * this.texture.width
		this.y = this.y - delta * 1e-2
		// this.x = x - 20

		super.update(engine, delta)

		if (this.y < y - this.texture.height || this.y > y + Game.drawHeight) {
			this.y = y
		}

	}

	draw(ctx: CanvasRenderingContext2D, delta: number) {

		for (let i = 0; i < Math.ceil(Game.drawWidth / this.texture.width) + 5; ++i) {
			this.currentDrawing.draw(ctx, this.x + i * this.texture.width, this.y)
			this.currentDrawing.draw(ctx, this.x + i * this.texture.width, this.y + this.texture.height)
		}

	}
}