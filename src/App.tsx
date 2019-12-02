import React, { useState } from "react";
import ScreenProps from "./components/screens/ScreenProps";
import GameScreen from "./components/screens/GameScreen";
import OptionsScreen from "./components/screens/OptionsScreen";
import SaveSelectionScreen from "./components/screens/SaveSelectionScreen";
import TitleScreen from "./components/screens/TitleScreen";
import "./App.css";

export default () => {
    const [screen, setScreen] = useState(process.env.FL_SCREEN || "title");

    const screens: { [key: string]: (props: ScreenProps) => JSX.Element } = {
        title: TitleScreen,
        saveSelection: SaveSelectionScreen,
        game: GameScreen,
        options: OptionsScreen,
    };
    const Screen =
        screens[screen] || console.error(`Screen: ${screen} does not exist.`);

    return (
        <div className="App">
            <Screen setScreen={setScreen} />
        </div>
    );
};
