import React, { useEffect, useState } from 'react';
import { withTheme } from '@rjsf/core';
import { Theme as MaterialUITheme } from '@rjsf/material-ui';
import './App.css';
import { JSONSchema7 } from 'json-schema';
import Box from '@material-ui/core/Box';
import Button from '@material-ui/core/Button';
const SERVER_IP = 'localhost';
const ws = new WebSocket(`ws://${SERVER_IP}:8080`);

const Form = withTheme<Record<any, string>>(MaterialUITheme);

enum Command {
  Reset,
  Init,
}

interface Data {
  editable?: Record<string, any>;
  non_editable?: Record<string, any>;
}

interface InitData {
  data: Data;
  schema: Schema;
}

interface Message {
  Data: Data;
  Command: Command;
  Init: InitData;
  Error: string;
}

interface Schema {
  editable: JSONSchema7;
  non_editable: JSONSchema7;
}

function update(editable: Record<string, any>) {
  ws.send(JSON.stringify({ Data: { editable } }));
}

function Container(props: { children: JSX.Element }): JSX.Element {
  return <div className="App">{props.children}</div>;
}

let lastUpdate = new Date().getTime();

function App() {
  const [editableData, setEditableData] = useState<
    Record<string, any> | undefined
  >();
  const [noneditableData, setNonEditableData] = useState<
    Record<string, any> | undefined
  >();
  const [error, setError] = useState<string | undefined>();
  const [schema, setSchema] = useState<Schema | undefined>();
  useEffect(() => {
    ws.onopen = () => {
      ws.send(JSON.stringify({ Command: Command.Init }));
    };
  });
  useEffect(() => {
    ws.onmessage = (m) => {
      const message: Message = JSON.parse(m.data);
      if (message.Data) {
        if (message.Data.editable) {
          console.log('Received  data!');
          setEditableData(message.Data.editable);
        }
        if (message.Data.non_editable) {
          const deltaTime = lastUpdate - new Date().getTime();
          if (deltaTime > 500) {
            setNonEditableData(message.Data.non_editable);
            lastUpdate = new Date().getTime();
          }
        }
      }
      if (message.Init) {
        setSchema(message.Init.schema);
        setEditableData(message.Init.data.editable);
        setNonEditableData(message.Init.data.non_editable);
      }
      if (message.Error) {
        setError(error);
      }
    };
  });

  if (!schema || !editableData) {
    return (
      <Container>
        <h1>Loading..</h1>
      </Container>
    );
  }

  return (
    <Container>
      <>
        <h1>{error}</h1>
        <Form formData={noneditableData} schema={schema.non_editable} readonly>
          <></>
        </Form>
        <Form
          noHtml5Validate
          formData={editableData}
          schema={schema.editable}
          onChange={(e, _) => {
            setEditableData(e.formData);
            if (e.errors.length === 0) update(e.formData);
          }}
        >
          {null}
        </Form>
      </>
    </Container>
  );
}

export default App;