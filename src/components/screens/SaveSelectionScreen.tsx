import React from "react";
import ScreenProps from "./ScreenProps";
import stylesMenu from "../../styles/menu.module.css";
import BackButton from "../BackButton";

export default (props: ScreenProps) => {
    return (
        <div className={`${stylesMenu.Menu} full-screen`}>
            <BackButton onClick={() => props.setScreen("title")} />
            <div className={stylesMenu.Title}>Select Save</div>
            <div className={stylesMenu.Buttons}>
                <button className="btn" onClick={() => props.setScreen("game")}>
                    Save Slot 1
                </button>
                <button className="btn" onClick={() => props.setScreen("game")}>
                    Save Slot 2
                </button>
                <button className="btn" onClick={() => props.setScreen("game")}>
                    Save Slot 3
                </button>
            </div>
        </div>
    );
};
