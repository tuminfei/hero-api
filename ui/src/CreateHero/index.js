import React from 'react';
import { connect } from 'react-redux';
import { createStructuredSelector } from 'reselect';
import { bindActionCreators } from 'redux';
import { Formik, Form, Field, ErrorMessage } from 'formik';
import {
  ActionCreators as HeroesActionCreators,
  Selectors as HeroesSelectors,
} from '../duck';
import './CreateHero.css';
import { Link } from 'react-router-dom';

const FormField = ({ label, fieldName, type }) => (
  <div className="form-field-container">
    <div className="form-field">
      <span className="form-field-label">{label}:</span>
      <Field className="form-field-input" type={type} name={fieldName} />
    </div>
    <ErrorMessage name={fieldName} component="div" />
  </div>
);

const sanitize = values => ({
  ...values,
  age: parseInt(values.age, 10),
});

class CreateHero extends React.Component {
  render() {
    const {
      actions: { createHeroRequest },
    } = this.props;
    return (
      <div>
        <Link to="/">Back</Link>
        <h1>Create Hero</h1>
        <Formik
          initialValues={{ name: '', identity: '', hometown: '', age: '' }}
          validate={values => {
            let errors = {};
            if (!values.name) {
              errors.name = 'Required';
            }
            if (!values.identity) {
              errors.identity = 'Required';
            }
            if (!values.hometown) {
              errors.hometown = 'Required';
            }
            if (!values.age) {
              errors.age = 'Required';
            }
            return errors;
          }}
          onSubmit={(values, { setSubmitting }) => {
            setTimeout(() => {
              createHeroRequest(sanitize(values));
              setSubmitting(false);
            }, 400);
          }}
        >
          {({ isSubmitting }) => (
            <Form>
              <FormField label="Name" fieldName="name" type="text" />
              <FormField label="Identity" fieldName="identity" type="text" />
              <FormField label="Hometown" fieldName="hometown" type="text" />
              <FormField label="Age" fieldName="age" type="text" />
              <button type="submit" disabled={isSubmitting}>
                Submit
              </button>
            </Form>
          )}
        </Formik>
      </div>
    );
  }
}

const mapStateToProps = createStructuredSelector({
  loading: HeroesSelectors.loadingSelector,
  error: HeroesSelectors.errorSelector,
});

const mapDispatchToProps = dispatch => ({
  actions: bindActionCreators(
    {
      createHeroRequest: HeroesActionCreators.createHeroRequest,
    },
    dispatch
  ),
});

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateHero);
