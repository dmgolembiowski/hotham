import React, { Suspense } from 'react';
import styled from 'styled-components';
import { Entity } from '../App';
import { Timeline } from './Timeline';
import { Viewer } from './Viewer';

const Container = styled.div`
  display: flex;
  flex: 3;
  flex-direction: column;
  position: relative;
`;

interface Props {
  setFrame: (n: number) => void;
  frame: number;
  maxFrames: number;
  entities: Record<number, Entity>;
}

export function LeftPanel(props: Props): JSX.Element {
  return (
    <Container>
      <Suspense fallback={null}>
        <Viewer entities={props.entities} />
      </Suspense>
      <Timeline {...props} />
    </Container>
  );
}