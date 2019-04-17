import { Actor, Engine, Color, Label, TextAlign } from "excalibur";
import { Config, Game } from "../glob/global";

export class DrowdownItem extends Actor {

	private label!: Label

	constructor(public readonly value: string,
		public readonly dropdown: Dropdown) {

		super(0, 0, Config.Dropdown.Width, Config.DropdownItem.Height, new Color(0, 0, 0))

		const label = new Label(value, 0, 0, "museo-sans, arial")
		this.add(label)
		label.color = new Color(255, 255, 255)
		label.fontSize = 16
		label.opacity = 0.8
		label.textAlign = TextAlign.Center
		label.y = 10
		this.label = label

		this.opacity = 0.4

		this.on("pointerdown", () => {

			if (this.visible) {
				this.dropdown.setSelected(this)
			}

		})

		this.on("pointerenter", () => {

			this.opacity = 0.8

		})

		this.on("pointerleave", () => {

			this.opacity = 0.4

		})

	}

}

export class Dropdown extends Actor {

	private label!: Label
	private readonly opts: DrowdownItem[] = []
	private idx: number = 0
	private open: boolean = false

	constructor(x?: number, y?: number) {

		super(x, y, Config.Dropdown.Width, Config.Dropdown.Height, new Color(131, 157, 99))

		const label = new Label("", 0, 0, "museo-sans, arial")
		this.add(label)
		label.color = new Color(255, 255, 255)
		label.fontSize = 16
		label.opacity = 0.8
		label.textAlign = TextAlign.Center
		label.y = 10
		this.label = label

		this.opacity = 0.6

		this.on("pointerup", () => {

			this.toggleDraw()

		})

	}

	addOptions(...options: string[]) {

		for (let i in options) {

			const j: number = Number(i)

			const opt = new DrowdownItem(options[j], this)
			this.opts.push(opt)
			Game.add(opt)

			opt.x = this.x
			opt.y = (Config.Dropdown.Height +
				Config.DropdownItem.Height) / 2 +
				j * Config.DropdownItem.Height + this.y

			opt.visible = false

		}

	}

	update(engine: Engine, delta: number) {

		super.update(engine, delta)

		if (this.idx < this.opts.length) {

			this.label.text = this.getValue()

		}

	}

	toggleDraw() {

		if (this.open) {

			for (const opt of this.opts) {
				opt.visible = false
			}
			this.open = false

		} else {

			for (const opt of this.opts) {
				opt.visible = true
			}
			this.open = true

		}

	}

	setSelected(option: DrowdownItem) {

		this.idx = this.opts.indexOf(option)

		this.toggleDraw()

	}

	getValue() {

		return this.opts[this.idx].value

	}

}