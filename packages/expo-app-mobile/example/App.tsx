import ExpoAppMobile from 'expo-app-mobile';
import { useEvent } from 'expo';
import { useState } from 'react';
import { Button, ScrollView, Text, View } from 'react-native';

export default function App() {
  const [delayedResult, setDelayedResult] = useState<number | null>(null);
  const tickPayload = useEvent(ExpoAppMobile, 'onTick');

  return (
    <View style={styles.container}>
      <ScrollView style={styles.container}>
        <Text style={styles.header}>Module API Example</Text>
        <Group name="Sync functions">
          <FunctionDemo title="add(1, 2)" call={() => ExpoAppMobile.add(1, 2)} />
          <FunctionDemo title="subtract(5, 3)" call={() => ExpoAppMobile.subtract(5, 3)} />
          <FunctionDemo title="multiply(4, 3)" call={() => ExpoAppMobile.multiply(4, 3)} />
          <FunctionDemo title="divide(10, 2)" call={() => ExpoAppMobile.divide(10, 2)} />
          <FunctionDemo title="divide(10, 0)" call={() => ExpoAppMobile.divide(10, 0)} />
        </Group>
        <Group name="Background task">
          <Button title="Start" onPress={() => ExpoAppMobile.startBackgroundTask()} />
          <Button title="Stop" onPress={() => ExpoAppMobile.stopBackgroundTask()} />
          <Text>Tick: {tickPayload?.count ?? '—'}</Text>
        </Group>
        <Group name="Async functions">
          <Button
            title="delayedAdd(1, 2, 1000ms)"
            onPress={async () => {
              setDelayedResult(null);
              const start = Date.now();
              const result = await ExpoAppMobile.delayedAdd(1, 2, 1000);
              console.warn(`[delayedAdd] took ${Date.now() - start}ms`);
              setDelayedResult(result);
            }}
          />
          <Text>Result: {delayedResult ?? '—'}</Text>
        </Group>
      </ScrollView>
    </View>
  );
}

function FunctionDemo(props: { title: string; call: () => number | null }) {
  const [result, setResult] = useState<number | null>(null);
  return (
    <View style={styles.demo}>
      <Button title={props.title} onPress={() => setResult(props.call())} />
      <Text>Result: {result ?? '—'}</Text>
    </View>
  );
}

function Group(props: { name: string; children: React.ReactNode }) {
  return (
    <View style={styles.group}>
      <Text style={styles.groupHeader}>{props.name}</Text>
      {props.children}
    </View>
  );
}

const styles = {
  header: {
    fontSize: 30,
    margin: 20,
  },
  groupHeader: {
    fontSize: 20,
    marginBottom: 20,
  },
  group: {
    margin: 20,
    backgroundColor: '#fff',
    borderRadius: 10,
    padding: 20,
  },
  demo: {
    marginBottom: 12,
  },
  container: {
    flex: 1,
    backgroundColor: '#eee',
    paddingTop: 60,
  },
};
