import { call, put, takeEvery, fork } from 'redux-saga/effects';
import Api from './api';

const Actions = {
  FETCH_HEROES_REQUEST: 'FETCH_HEROES_REQUEST',
  FETCH_HEROES_SUCCESS: 'FETCH_HEROES_SUCCESS',
  FETCH_HEROES_FAILURE: 'FETCH_HEROES_FAILURE',

  FETCH_HERO_REQUEST: 'FETCH_HERO_REQUEST',
  FETCH_HERO_SUCCESS: 'FETCH_HERO_SUCCESS',
  FETCH_HERO_FAILURE: 'FETCH_HERO_FAILURE',

  CREATE_HERO_REQUEST: 'CREATE_HERO_REQUEST',
  CREATE_HERO_SUCCESS: 'CREATE_HERO_SUCCESS',
  CREATE_HERO_FAILURE: 'CREATE_HERO_FAILURE',

  UPDATE_HERO_REQUEST: 'UPDATE_HERO_REQUEST',
  UPDATE_HERO_SUCCESS: 'UPDATE_HERO_SUCCESS',
  UPDATE_HERO_FAILURE: 'UPDATE_HERO_FAILURE',

  DELETE_HERO_REQUEST: 'DELETE_HERO_REQUEST',
  DELETE_HERO_SUCCESS: 'DELETE_HERO_SUCCESS',
  DELETE_HERO_FAILURE: 'DELETE_HERO_FAILURE',
};

export const ActionCreators = {
  fetchHeroesRequest: id => ({
    type: Actions.FETCH_HEROES_REQUEST,
    payload: id,
  }),
  fetchHeroesSuccess: heroes => ({
    type: Actions.FETCH_HEROES_SUCCESS,
    payload: heroes,
  }),
  fetchHeroesFailure: error => ({
    type: Actions.FETCH_HEROES_FAILURE,
    payload: error,
  }),

  fetchHeroRequest: id => ({
    type: Actions.FETCH_HERO_REQUEST,
    payload: id,
  }),
  fetchHeroSuccess: payload => ({
    type: Actions.FETCH_HERO_SUCCESS,
    payload,
  }),
  fetchHeroFailure: error => ({
    type: Actions.FETCH_HERO_FAILURE,
    payload: error,
  }),

  createHeroRequest: hero => ({
    type: Actions.CREATE_HERO_REQUEST,
    payload: hero,
  }),
  createHeroSuccess: () => ({
    type: Actions.CREATE_HERO_SUCCESS,
  }),
  createHeroFailure: error => ({
    type: Actions.CREATE_HERO_FAILURE,
    payload: error,
  }),

  updateHeroRequest: hero => ({
    type: Actions.UPDATE_HERO_REQUEST,
    payload: hero,
  }),
  updateHeroSuccess: payload => ({
    type: Actions.UPDATE_HERO_SUCCESS,
    payload,
  }),
  updateHeroFailure: error => ({
    type: Actions.UPDATE_HERO_FAILURE,
    payload: error,
  }),

  deleteHeroRequest: id => ({
    type: Actions.DELETE_HERO_REQUEST,
    payload: id,
  }),
  deleteHeroSuccess: payload => ({
    type: Actions.DELETE_HERO_SUCCESS,
    payload,
  }),
  deleteHeroFailure: error => ({
    type: Actions.DELETE_HERO_FAILURE,
    payload: error,
  }),
};

export const REDUCER_KEY = 'heroes';

export const Selectors = {
  loadingSelector: state => state[REDUCER_KEY].loading,
  heroesSelector: state => state[REDUCER_KEY].heroes,
  heroSelector: state => state[REDUCER_KEY].hero,
  errorSelector: state => state[REDUCER_KEY].error,
};

const initialState = {
  loading: false,
  heroes: null,
  error: null,
  hero: null,
};

export default function heroesReducer(state = initialState, action) {
  switch (action.type) {
    case Actions.FETCH_HEROES_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
        heroes: action.payload,
      };

    case Actions.FETCH_HERO_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
        hero: action.payload,
      };

    case Actions.CREATE_HERO_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
      };

    case Actions.UPDATE_HERO_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
        hero: action.payload,
      };

    case Actions.DELETE_HERO_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
        heroes: state.heroes.filter(h => h.id !== action.payload.id),
        hero: null,
      };

    case Actions.FETCH_HEROES_REQUEST:
    case Actions.FETCH_HERO_REQUEST:
    case Actions.CREATE_HERO_REQUEST:
    case Actions.UPDATE_HERO_REQUEST:
    case Actions.DELETE_HERO_REQUEST:
      return {
        ...state,
        loading: true,
        error: null,
      };

    case Actions.FETCH_HEROES_FAILURE:
    case Actions.FETCH_HERO_FAILURE:
    case Actions.CREATE_HERO_FAILURE:
    case Actions.DELETE_HERO_FAILURE:
      return {
        ...state,
        loading: false,
        heroes: null,
        error: action.payload,
      };

    default:
      return state;
  }
}

function* fetchHeroesSaga() {
  yield takeEvery(Actions.FETCH_HEROES_REQUEST, fetchHeroes);
}
function* fetchHeroes() {
  const { response, error } = yield call(Api.fetchHeroes);

  if (response) {
    yield put(ActionCreators.fetchHeroesSuccess(response));
  } else {
    yield put(ActionCreators.fetchHeroesFailure(error));
  }
}

function* fetchHeroSaga() {
  yield takeEvery(Actions.FETCH_HERO_REQUEST, fetchHero);
}
function* fetchHero(action) {
  const { payload } = action;
  const { response, error } = yield call(Api.fetchHero, payload);

  if (response) {
    yield put(ActionCreators.fetchHeroSuccess(response));
  } else {
    yield put(ActionCreators.fetchHeroFailure(error));
  }
}

function* createHeroSaga() {
  yield takeEvery(Actions.CREATE_HERO_REQUEST, createHero);
}

function* createHero(action) {
  const { payload } = action;
  const { response, error } = yield call(Api.createHero, payload);

  if (response) {
    yield put(ActionCreators.createHeroSuccess(response));
    yield put(ActionCreators.fetchHeroesRequest());
  } else {
    yield put(ActionCreators.createHeroFailure(error));
  }
}

function* updateHeroSaga() {
  yield takeEvery(Actions.UPDATE_HERO_REQUEST, updateHero);
}
function* updateHero(action) {
  const { payload } = action;
  const { response, error } = yield call(Api.updateHero, payload);

  if (response) {
    yield put(ActionCreators.updateHeroSuccess(response));
    yield put(ActionCreators.fetchHeroesRequest());
  } else {
    yield put(ActionCreators.updateHeroFailure(error));
  }
}

function* deleteHeroSaga() {
  yield takeEvery(Actions.DELETE_HERO_REQUEST, deleteHero);
}
function* deleteHero(action) {
  const { payload } = action;
  const { response, error } = yield call(Api.deleteHero, payload);

  if (response) {
    yield put(ActionCreators.deleteHeroSuccess(response));
    yield put(ActionCreators.fetchHeroesRequest());
  } else {
    yield put(ActionCreators.deleteHeroFailure(error));
  }
}

export const sagas = [
  fork(fetchHeroesSaga),
  fork(fetchHeroSaga),
  fork(createHeroSaga),
  fork(updateHeroSaga),
  fork(deleteHeroSaga),
];
