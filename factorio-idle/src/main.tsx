import React from 'react'
import ReactDOM from 'react-dom/client'
// import App from './App.tsx'
import './index.css'
import init from 'game-core';
import {GameStateContext} from "./GameState.tsx";
import {GameState} from "game-core";

let gameState : GameState;
init().then(() => {
    console.log("initialized");
    gameState = GameState.new()
}).then(() => {
    ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
        <GameStateContext gameState={gameState}/>,
    )
})


