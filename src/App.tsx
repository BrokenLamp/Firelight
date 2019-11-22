import React, { useState } from "react";
import match from "./patternMatch";
import GameScreen from "./components/screens/GameScreen";
import TitleScreen from "./components/screens/TitleScreen";
import "./App.css";

const core: any = window.require("core");

export default () => {
    const [screen, setScreen] = useState(process.env.FL_SCREEN || "title");
    const screenComponent = match<string, JSX.Element>(screen)(
        ["game", () => <GameScreen setScreen={setScreen} />],
        ["title", () => <TitleScreen setScreen={setScreen} />],
    );
    return <div className="App">{screenComponent}</div>;
};
