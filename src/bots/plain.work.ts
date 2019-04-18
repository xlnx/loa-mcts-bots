import { my_plain_solution } from "../../pkg/ai_frontend";

console.log("WebWorker for my plain solution")

addEventListener("message", (msg) => {

	console.log(msg)

	postMessage(msg)
	// const res = my_plain_solution(this.turn, Int32Array.from(board))

})