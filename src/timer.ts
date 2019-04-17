export class Timer {

	private readonly fcn: () => void
	private stopped: boolean = false

	constructor(fn: () => void, interval: number) {

		this.fcn = () => {
			if (!this.stopped) {
				fn()
				setTimeout(this.fcn, interval)
			}
		}
		setTimeout(this.fcn, 0)

	}

	cancel() {
		this.stopped = true
	}

}