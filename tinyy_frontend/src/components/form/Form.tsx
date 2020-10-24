import React from 'react';
import './form.css';
import logo from '../../logo.svg';
import { createTinyUrl } from '../../api';

interface FormProps {}

interface FormState {
  value: string,
  tinyUrl?: string
}

class UrlForm extends React.Component<FormProps, FormState>{

  constructor(props: FormProps) {
    super(props);

    this.state = {value: ""};

    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  private handleChange(event: React.FormEvent<HTMLInputElement>) {
    this.setState({value: (event.target as HTMLInputElement).value});
  }

  private async handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (!this.state.value)
      return

    let tinyUrl = await createTinyUrl(this.state.value);
    this.setState({value: "", tinyUrl: tinyUrl});

    console.log(this.state.tinyUrl);
  }

  render() {
    return (
      <div className="UrlForm">
        <img src={logo} className="UrlForm-logo" alt="logo" />
        <form className="UrlForm-form" onSubmit={this.handleSubmit}>
          <input id="url" className="UrlForm-input" value={this.state.value} onChange={this.handleChange} />
          <button className="UrlForm-btn" type="submit"><i className="UrlForm-arrow"></i></button>
        </form>
      </div>
    )
  }
}

export default UrlForm;
