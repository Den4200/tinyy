import React from 'react';
import './urltable.css';
import { TinyUrl } from '../../api';

interface UrlTableProps {
  tinyUrls: TinyUrl[]
}

function UrlTable(props: UrlTableProps) {
  if (props.tinyUrls.length > 0) {
    return (
      <table className="UrlTable">
        {props.tinyUrls.map(tinyUrl => (
            <tr>
              <td>
                <a className="UrlTable-url" href={tinyUrl.tinyUrl}>{tinyUrl.tinyUrl}</a>
              </td>
              <td style={{textAlign: "right"}}>
                <a className="UrlTable-url" href={tinyUrl.url}>{tinyUrl.url}</a>
              </td>
            </tr>
        ))}
      </table>
    );
  }
  return null;
}

export default UrlTable;
