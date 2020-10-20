import React from 'react';
import './form.css';

class UrlForm extends React.Component{

  private handleSubmit = async (
      event: React.FormEvent<HTMLFormElement>
): Promise<void> => {
    event.preventDefault();
    // send to api
  }

  render() {
    return (
      <form className="UrlForm" onSubmit={this.handleSubmit}>
        <input id="url" className="UrlForm-input" />
        <button className="UrlForm-btn" type="submit"><i className="UrlForm-arrow"></i></button>
      </form>
    )
  }
}

export default UrlForm;
