import React from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import CardActionArea from '@material-ui/core/CardActionArea';
import CardActions from '@material-ui/core/CardActions';
import CardContent from '@material-ui/core/CardContent';
import CardMedia from '@material-ui/core/CardMedia';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';

const styles = {
  card: {
    maxWidth: 345,
  },
  media: {
    height: 140,
  },
};

function HeroCard(props) {
  const { classes, hero } = props;
  const { name, identity, hometown, age, image_url } = hero;
  return (
    <Card className={classes.card}>
      <CardActionArea>
        <CardMedia
          className={classes.media}
          image={image_url}
          title="Contemplative Reptile"
        />
        <CardContent>
          <Typography gutterBottom variant="h5" component="h2">
            {name}
          </Typography>
          <Typography component="p">
            {name} is the alias for {identity}, a superhero in {hometown}.
          </Typography>
        </CardContent>
      </CardActionArea>
      <CardActions>
        <Button size="small" color="primary">
          Share
        </Button>
        <Button size="small" color="primary">
          Learn More
        </Button>
      </CardActions>
    </Card>
  );
}

HeroCard.propTypes = {
  classes: PropTypes.object.isRequired,
  hero: PropTypes.shape({
    name: PropTypes.string.isRequired,
    identity: PropTypes.string.isRequired,
    hometown: PropTypes.string.isRequired,
    age: PropTypes.number.isRequired,
  }).isRequired,
};

export default withStyles(styles)(HeroCard);
