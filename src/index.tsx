import React from 'react';
import ReactDOM from 'react-dom';
import { listen } from '@tauri-apps/api/event';
import './index.css';
import App from './App';

main();

async function main() {
  console.log('waiting for bootstrap message');
  const unlisten = await listen('bootstrap', ({ payload }) => {
    console.log('Bootstrap happened', payload);

    ReactDOM.render(
      <React.StrictMode>
        <App />
      </React.StrictMode>,
      document.getElementById('root')
    );
  });

  unlisten();
}
