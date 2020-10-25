import React from 'react';
import './form.css';
import UrlTable from '../url_table/UrlTable';
import logo from '../../logo.svg';
import { createTinyUrl, TinyUrl, TinyStatus } from '../../api';

interface FormProps {}

interface FormState {
  value: string,
  tinyUrls: TinyUrl[]
}

class UrlForm extends React.Component<FormProps, FormState>{

  constructor(props: FormProps) {
    super(props);

    this.state = {value: "", tinyUrls: []};

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

    let tinyUrlSpec = await createTinyUrl(this.state.value);

    if (tinyUrlSpec.status === TinyStatus.Ok) {
      this.setState(prevState => ({
        value: "",
        tinyUrls: [...prevState.tinyUrls, tinyUrlSpec.tinyUrl!]
      }));
    } else {
      console.log(`Status ${tinyUrlSpec.status}`)
    }
  }

  render() {
    return (
      <div className="UrlForm">
        <img src={logo} className="UrlForm-logo" alt="logo" />
        <form className="UrlForm-form" onSubmit={this.handleSubmit}>
          <input id="url" className="UrlForm-input" value={this.state.value} onChange={this.handleChange} placeholder="Link here.." />

          <button className="UrlForm-btn" aria-label="Shorten URL" type="submit">
            <i className="UrlForm-arrow"></i>
          </button>
        </form>
        <UrlTable tinyUrls={this.state.tinyUrls} />
      </div>
    )
  }
}

export default UrlForm;
