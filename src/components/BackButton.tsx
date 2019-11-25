import React from "react";
import styles from "./BackButton.module.css";

interface BackButtonProps {
    onClick: () => void;
}

export default (props: BackButtonProps) => {
    return (
        <div className={styles.BackButton} onClick={props.onClick}>
            ⧼&nbsp;
        </div>
    );
};
