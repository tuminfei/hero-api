import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import './index.css';
import AppHeader from './AppHeader';
import * as serviceWorker from './serviceWorker';
import './App.css';

import Home from './Home';
import CreateHero from './CreateHero';
import HeroDetail from './HeroDetail';
import { all } from 'redux-saga/effects';
import { Route, Switch } from 'react-router';
import logger from 'redux-logger';
import {
  ConnectedRouter,
  connectRouter,
  routerMiddleware,
} from 'connected-react-router';
import { createBrowserHistory } from 'history';
import { applyMiddleware, compose, createStore, combineReducers } from 'redux';
import createSagaMiddleware from 'redux-saga';
import heroesReducer, {
  REDUCER_KEY as HEROES_REDUCER_KEY,
  sagas as heroesSagas,
} from './duck';

const history = createBrowserHistory();

const rootReducer = combineReducers({
  router: connectRouter(history),
  [HEROES_REDUCER_KEY]: heroesReducer,
});

let store;
const initialState = {};
const sagaMiddleware = createSagaMiddleware();

function getStoreEnhancers() {
  return compose(
    applyMiddleware(
      logger,
      routerMiddleware(history), // for dispatching history actions
      sagaMiddleware
    )
  );
}

export default function* allSagas() {
  yield all([...heroesSagas]);
}

function getStore() {
  if (!store) {
    store = createStore(rootReducer, initialState, getStoreEnhancers());
    sagaMiddleware.run(allSagas);
  }
  return store;
}

ReactDOM.render(
  <Provider store={getStore()}>
    <ConnectedRouter history={history}>
      <div className="app">
        <AppHeader />
        <Switch>
          <Route exact path="/" render={() => <Home />} />
          <Route exact path="/hero/:id" render={() => <HeroDetail />} />
          <Route exact path="/create" render={() => <CreateHero />} />
          <Route render={() => <div>Not Found</div>} />
        </Switch>
      </div>
    </ConnectedRouter>
  </Provider>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: http://bit.ly/CRA-PWA
serviceWorker.unregister();
