import React from 'react';
import logo from './logo.svg';
import './AppHeader.css';

export default React.memo(() => (
  <div className="App">
    <header className="App-header">
      <img src={logo} className="App-logo" alt="logo" />
    </header>
  </div>
));
