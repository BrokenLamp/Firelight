import React from "react";
import styles from "./TitleScreen.module.css";

export default (props: any) => {
    return (
        <div className={styles.TitleScreen}>
            <div className={styles.Title}>
                Firelight
            </div>
            <div className={styles.Buttons}>
                <button className="btn" onClick={() => props.setScreen("game")}>Play</button>
                <button className="btn">Options</button>
                <button className="btn">Quit</button>
            </div>
        </div>
    )
}
