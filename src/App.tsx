import React from "react";
import logo from "./logo.svg";
import "./App.css";

const core: any = window.require("core");

const App: React.FC = () => {
    return (
        <div className="App">
            <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <p>
                    Edit <code>src/App.tsx</code> and save to reload.
                </p>
                <a
                    className="App-link"
                    href="https://reactjs.org"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    {JSON.stringify(core.hello())}
                </a>
            </header>
        </div>
    );
};

export default App;
