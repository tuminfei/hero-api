import React from 'react';
import { connect } from 'react-redux';
import { createStructuredSelector } from 'reselect';
import { bindActionCreators } from 'redux';
import {
  Selectors as HeroesSelectors,
  ActionCreators as HeroesActionCreators,
} from '../duck';
import { Link } from 'react-router-dom';
import Button from '@material-ui/core/Button';
import HeroCard from '../HeroCard';

class Hero extends React.PureComponent {
  handleDelete = () => this.props.deleteHeroRequest(this.props.hero.id);

  render() {
    const { hero } = this.props;
    return (
      <div>
        <Link to={`/hero/${hero.id}`}>
          <HeroCard hero={hero} />
        </Link>
        <Button variant="contained" color="primary" onClick={this.handleDelete}>
          Delete
        </Button>
      </div>
    );
  }
}

const HeroList = ({ heroes, deleteHeroRequest }) => (
  <div>
    <h1>Superheroes</h1>
    {heroes.map((h, i) => (
      <Hero deleteHeroRequest={deleteHeroRequest} hero={h} key={i} />
    ))}
  </div>
);

class Home extends React.Component {
  componentDidMount() {
    const {
      actions: { fetchHeroesRequest },
    } = this.props;
    fetchHeroesRequest();
  }

  render() {
    const { heroes, loading, error } = this.props;
    const {
      actions: { deleteHeroRequest },
    } = this.props;
    console.log('heroes =', heroes);
    return (
      <div>
        <Link to="/create">Create New Hero</Link>
        {heroes && (
          <HeroList deleteHeroRequest={deleteHeroRequest} heroes={heroes} />
        )}
      </div>
    );
  }
}

const mapStateToProps = createStructuredSelector({
  loading: HeroesSelectors.loadingSelector,
  error: HeroesSelectors.errorSelector,
  heroes: HeroesSelectors.heroesSelector,
});

const mapDispatchToProps = dispatch => ({
  actions: bindActionCreators(
    {
      fetchHeroesRequest: HeroesActionCreators.fetchHeroesRequest,
      deleteHeroRequest: HeroesActionCreators.deleteHeroRequest,
    },
    dispatch
  ),
});

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Home);
