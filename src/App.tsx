import React, { useState } from "react";
import match from "./patternMatch";
import ScreenProps from "./components/screens/ScreenProps";
import GameScreen from "./components/screens/GameScreen";
import OptionsScreen from "./components/screens/OptionsScreen";
import TitleScreen from "./components/screens/TitleScreen";
import "./App.css";

const core: any = window.require("core");

export default () => {
    const [screen, setScreen] = useState(process.env.FL_SCREEN || "title");

    const screens: { [key: string]: (props: ScreenProps) => JSX.Element } = {
        game: GameScreen,
        options: OptionsScreen,
        title: TitleScreen,
    };
    const Screen =
        screens[screen] || console.error(`Screen: ${screen} does not exist.`);

    return (
        <div className="App">
            <Screen setScreen={setScreen} />
        </div>
    );
};
