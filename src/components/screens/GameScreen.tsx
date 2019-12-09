import React from "react";
import styles from "./GameScreen.module.css";

export default (props: any) => {
    return (
        <div className={styles.GameScreen}>
            <div className={styles.Head}>
                HEAD
            </div>
            <div className={styles.Log}>
                LOG
            </div>
            <div className={styles.Main}>
                MAIN
            </div>
            <div className={styles.Map}>
                MAP
            </div>
        </div>
    );
};
