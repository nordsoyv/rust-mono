import React, { useEffect, useState } from 'react';
//import {GameState} from "game-core";
import { App } from './App.tsx';
import { GameState } from '../game-core/pkg/game_core';

function useForceUpdate() {
  const [_value, setValue] = useState(0); // integer state
  return () => setValue((value) => ++value); // update the state to force render
}
export const GameStateContext = ({ gameState }: { gameState: GameState }) => {
  const forceUpdate = useForceUpdate();
  useEffect(() => {
    setInterval(() => {
      if (gameState != null) {
        gameState.tick(1);
        forceUpdate();
      }
    }, 100);
  }, []);

  return (
    <React.StrictMode>
      <App gameState={gameState} />
    </React.StrictMode>
  );
};
