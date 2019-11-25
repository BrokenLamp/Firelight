import React, { useEffect } from "react";
import stylesMenu from "../../styles/menu.module.css";
import core from "../../core";

export default (props: any) => {
    useEffect(() => {
        console.log(core.setSoundscape("title"));
    });
    return (
        <div className={`${stylesMenu.Menu} full-screen`}>
            <div className={stylesMenu.Title}>Firelight</div>
            <div className={stylesMenu.Buttons}>
                <button
                    className="btn"
                    onClick={() => props.setScreen("saveSelection")}
                >
                    Play
                </button>
                <button
                    className="btn"
                    onClick={() => props.setScreen("options")}
                >
                    Options
                </button>
                <button
                    className="btn"
                    onClick={() => core.setSoundscape("title")}
                >
                    Quit
                </button>
            </div>
        </div>
    );
};
