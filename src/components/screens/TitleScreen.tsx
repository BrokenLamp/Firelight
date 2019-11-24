import React, { useEffect } from "react";
import styles from "./TitleScreen.module.css";
import core from "../../core";

export default (props: any) => {
    useEffect(() => {
        console.log(core.setSoundscape("title"));
    });
    return (
        <div className={styles.TitleScreen}>
            <div className={styles.Title}>Firelight</div>
            <div className={styles.Buttons}>
                <button className="btn" onClick={() => props.setScreen("game")}>
                    Play
                </button>
                <button
                    className="btn"
                    onClick={() => props.setScreen("options")}
                >
                    Options
                </button>
                <button className="btn" onClick={() => alert("TODO.")}>
                    Quit
                </button>
            </div>
        </div>
    );
};
