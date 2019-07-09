const BASE_API_URL = 'http://localhost:8000';

const getHeaders = () => ({
  // "Content-Type": "application/json; charset=utf-8",
  'Content-Type': 'application/json',
});

export default class Api {
  static fetchHero(id) {
    return fetch(`${BASE_API_URL}/hero/${id}`, {
      method: 'GET',
      mode: 'cors',
      headers: getHeaders(),
    })
      .then(r => r.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }

  static fetchHeroes() {
    return fetch(`${BASE_API_URL}/hero`)
      .then(r => r.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }

  static createHero(h) {
    const body = JSON.stringify(h);
    console.log('body =', body);
    return fetch(`${BASE_API_URL}/hero`, {
      method: 'POST',
      mode: 'cors',
      headers: getHeaders(),
      body,
    })
      .then(r => r.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }

  static updateHero(h) {
    const body = JSON.stringify(h);
    return fetch(`${BASE_API_URL}/hero`, {
      method: 'PUT',
      mode: 'cors',
      headers: getHeaders(),
      body,
    })
      .then(r => r.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }

  static deleteHero(id) {
    return fetch(`${BASE_API_URL}/hero/${id}`, {
      method: 'DELETE',
      mode: 'cors',
      headers: getHeaders(),
    })
      .then(r => r.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }
}
