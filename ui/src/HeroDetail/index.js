import React from 'react';
import { connect } from 'react-redux';
import { createStructuredSelector } from 'reselect';
import { bindActionCreators } from 'redux';
import { withRouter } from 'react-router';
import {
  ActionCreators as HeroesActionCreators,
  Selectors as HeroesSelectors,
} from '../duck';
import { Link } from 'react-router-dom';
import HeroCard from '../HeroCard';

class HeroDetail extends React.Component {
  componentDidMount() {
    const {
      actions: { fetchHeroRequest },
      match: {
        params: { id: heroID },
      },
    } = this.props;
    console.log('requesting hero', heroID);
    fetchHeroRequest(heroID);
  }

  render() {
    const { hero, loading, error } = this.props;
    if (loading) {
      return <div>Loading</div>;
    }
    if (error) {
      return <div>{JSON.stringify(error)}</div>;
    }
    return (
      <div>
        <div>
          <Link to="/">Back</Link>
        </div>
        <div>{hero && <HeroCard hero={hero} />}</div>
      </div>
    );
  }
}

const mapStateToProps = createStructuredSelector({
  loading: HeroesSelectors.loadingSelector,
  error: HeroesSelectors.errorSelector,
  hero: HeroesSelectors.heroSelector,
});

const mapDispatchToProps = dispatch => ({
  actions: bindActionCreators(
    {
      fetchHeroRequest: HeroesActionCreators.fetchHeroRequest,
    },
    dispatch
  ),
});

export default withRouter(
  connect(
    mapStateToProps,
    mapDispatchToProps
  )(HeroDetail)
);
