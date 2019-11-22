import React, { useState } from "react";
import match from "./patternMatch";
import TitleScreen from "./components/screens/TitleScreen";
import "./App.css";

const core: any = window.require("core");

export default () => {
    const [screen, setScreen] = useState("title");
    const screenComponent = match<string, JSX.Element>(screen)([
        "title",
        () => <TitleScreen setScreen={setScreen} />,
    ]);
    return <div className="App">{screenComponent}</div>;
};
