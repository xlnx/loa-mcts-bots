export abstract class Bot {

	private static bots: Map<string, new (turn: number) => Bot> = new Map

	constructor(public readonly turn: number) { }

	abstract makeMove(board: number[]): Promise<{ x0: number, y0: number, x1: number, y1: number }>

	static register(name: string, bot: new (turn: number) => Bot) {

		if (this.bots.has(name)) {
			console.warn(`bot ${name} has already been registered.`)
		} else {
			this.bots.set(name, bot)
		}

	}

	static get(name: string) {

		return this.bots.get(name)

	}

	static list(): string[] {

		return Array.from(this.bots.keys())

	}

}
