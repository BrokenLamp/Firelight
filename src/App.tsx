import React from "react";
import EventLog from "./components/EventLog";
import "./App.css";

const core: any = window.require("core");

export default () => {
    return (
        <div className="App">
            <EventLog />
            <button className="input btn">{core.hello()}</button>
        </div>
    );
};
