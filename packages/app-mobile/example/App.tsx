import AppMobile from "app-mobile";
import { useState } from "react";
import { Button, ScrollView, Text, View } from "react-native";

export default function App() {
  const [addResult, setAddResult] = useState<number | null>(null);
  const [tickCount, setTickCount] = useState(0);
  const [delayedResult, setDelayedResult] = useState<number | null>(null);

  return (
    <View style={styles.container}>
      <ScrollView style={styles.container}>
        <Text style={styles.header}>AppMobile UniFFI Example</Text>
        <Group name="Sync Functions">
          <Button
            title="add(2, 3)"
            onPress={() => setAddResult(AppMobile.add(2, 3))}
          />
          {addResult !== null && <Text>AddResult: {addResult}</Text>}
        </Group>
        <Group name="Async Functions">
          <Button
            title="Delayed Add (2 + 3, 1s delay)"
            onPress={async () => {
              const result = await AppMobile.delayedAdd(2, 3, 1000);
              setDelayedResult(result);
            }}
          />
          {delayedResult !== null && <Text>DelayedAddResult: {delayedResult}</Text>}
        </Group>
        <Group name="Background Task">
          <Text>Tick count: {tickCount}</Text>
          <Button
            title="Start"
            onPress={() => {
              AppMobile.addListener("onTick", (event) => {
                setTickCount(event.count);
              });
              AppMobile.startBackgroundTask();
            }}
          />
          <Button
            title="Stop"
            onPress={() => {
              AppMobile.stopBackgroundTask();
              setTickCount(0);
            }}
          />
        </Group>
      </ScrollView>
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
    backgroundColor: "#fff",
    borderRadius: 10,
    padding: 20,
  },
  container: {
    flex: 1,
    paddingTop: 24,
    backgroundColor: "#eee",
  },
};
