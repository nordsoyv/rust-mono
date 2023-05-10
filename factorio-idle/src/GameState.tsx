import React from "react";
import {GameState} from "game-core";
import {App} from "./App.tsx";

export class GameStateContext extends React.Component<{ gameState: GameState | null }, {}> {

    constructor(props: { gameState: GameState | null }) {
        super(props);
    }
    componentDidMount() {
        setInterval(() => {
            if (this.props.gameState != null) {
                this.props.gameState.tick(1);
                this.forceUpdate()
            }
        }, 100)
    }

    render() {
        return <React.StrictMode>
            <App gameState={this.props.gameState}/>
        </React.StrictMode>
    }
}
