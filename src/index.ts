import { Loader, Vector } from "excalibur";
import { Config, Resources, Game } from "./global";
import { Background } from "./background";
import { UI } from "./uiglobal";
import { GameLogic } from "./gamelogic";

const loader = new Loader()
for (const rc in Resources) {
	loader.addResource(Resources[rc])
}

import "./bots/idiot"

Game.start(loader).then(() => {

	const panelHeight = Config.Cell.Height * 8

	const scale = 1.55 - panelHeight / Game.drawHeight

	const background = new Background(Resources.BackgroundTexture)
	Game.add(background)

	Game.add(UI.Menu)
	Game.add(UI.Grid)
	Game.add(UI.Stat)

	Game.currentScene.camera.zoom(scale, 0)
	Game.currentScene.camera.move(new Vector(.5, .5), 0)

	GameLogic.init()
})

