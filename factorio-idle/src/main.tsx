// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import init, { GameState } from 'game-core';
import { GameStateContext } from './GameState.tsx';

let gameState: GameState;
init()
  .then(() => {
    console.log('initialized');
    gameState = GameState.new();
  })
  .then(() => {
    ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
      <GameStateContext gameState={gameState} />,
    );
  });
