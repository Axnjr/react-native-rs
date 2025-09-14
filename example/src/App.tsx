import React, { useState } from 'react';
import {
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  TextInput,
  TouchableOpacity,
  View,
  Alert,
} from 'react-native';
import { RustBridge } from 'react-native-rs';

interface Result {
  type: string;
  data: any;
  time: number;
}

const App = (): JSX.Element => {
  const [fibInput, setFibInput] = useState('10');
  const [hashInput, setHashInput] = useState('Hello, World!');
  const [sortInput, setSortInput] = useState('3,1,4,1,5,9,2,6,5,3,5');
  const [primeInput, setPrimeInput] = useState('100');
  const [results, setResults] = useState<Result[]>([]);
  const [loading, setLoading] = useState(false);

  const addResult = (type: string, data: any, time: number) => {
    setResults(prev => [{ type, data, time }, ...prev.slice(0, 9)]);
  };

  const runFibonacci = async () => {
    try {
      setLoading(true);
      const start = Date.now();
      const result = await RustBridge.execute({
        cmd: 'fibonacci',
        params: { n: parseInt(fibInput) || 10 }
      });
      const time = Date.now() - start;
      addResult('Fibonacci', result, time);
    } catch (error) {
      Alert.alert('Error', `Fibonacci failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const runHash = async () => {
    try {
      setLoading(true);
      const start = Date.now();
      const result = await RustBridge.execute({
        cmd: 'hash_data',
        params: { data: hashInput }
      });
      const time = Date.now() - start;
      addResult('Hash', result, time);
    } catch (error) {
      Alert.alert('Error', `Hash failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const runSort = async () => {
    try {
      setLoading(true);
      const numbers = sortInput.split(',').map(n => parseInt(n.trim())).filter(n => !isNaN(n));
      const start = Date.now();
      const result = await RustBridge.execute({
        cmd: 'sort_numbers',
        params: { numbers, algorithm: 'parallel' }
      });
      const time = Date.now() - start;
      addResult('Sort', result, time);
    } catch (error) {
      Alert.alert('Error', `Sort failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const runPrimes = async () => {
    try {
      setLoading(true);
      const start = Date.now();
      const result = await RustBridge.execute({
        cmd: 'find_primes',
        params: { limit: parseInt(primeInput) || 100, use_parallel: true }
      });
      const time = Date.now() - start;
      addResult('Primes', result, time);
    } catch (error) {
      Alert.alert('Error', `Primes failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const runBenchmark = async () => {
    try {
      setLoading(true);
      
      // Run multiple operations in parallel
      const operations = [
        RustBridge.execute({ cmd: 'fibonacci', params: { n: 35 } }),
        RustBridge.execute({ cmd: 'find_primes', params: { limit: 10000, use_parallel: true } }),
        RustBridge.execute({ cmd: 'sort_numbers', params: { 
          numbers: Array.from({length: 10000}, () => Math.floor(Math.random() * 1000)),
          algorithm: 'parallel' 
        }}),
      ];

      const start = Date.now();
      const results = await Promise.all(operations);
      const time = Date.now() - start;
      
      addResult('Benchmark', {
        fibonacci: results[0],
        primes_count: results[1].count,
        sort_length: results[2].input_length,
        total_operations: 3
      }, time);
    } catch (error) {
      Alert.alert('Error', `Benchmark failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar barStyle="dark-content" />
      <ScrollView contentInsetAdjustmentBehavior="automatic" style={styles.scrollView}>
        <View style={styles.header}>
          <Text style={styles.title}>React Native ❤️ Rust</Text>
          <Text style={styles.subtitle}>High-Performance Computing Bridge</Text>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>🔢 Fibonacci</Text>
          <TextInput
            style={styles.input}
            value={fibInput}
            onChangeText={setFibInput}
            placeholder="Enter number (e.g., 10)"
            keyboardType="numeric"
          />
          <TouchableOpacity style={styles.button} onPress={runFibonacci} disabled={loading}>
            <Text style={styles.buttonText}>Calculate</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>🔐 SHA-256 Hash</Text>
          <TextInput
            style={styles.input}
            value={hashInput}
            onChangeText={setHashInput}
            placeholder="Enter text to hash"
          />
          <TouchableOpacity style={styles.button} onPress={runHash} disabled={loading}>
            <Text style={styles.buttonText}>Hash</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>🔄 Parallel Sort</Text>
          <TextInput
            style={styles.input}
            value={sortInput}
            onChangeText={setSortInput}
            placeholder="Enter numbers (comma separated)"
          />
          <TouchableOpacity style={styles.button} onPress={runSort} disabled={loading}>
            <Text style={styles.buttonText}>Sort</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>🔢 Prime Numbers</Text>
          <TextInput
            style={styles.input}
            value={primeInput}
            onChangeText={setPrimeInput}
            placeholder="Enter limit (e.g., 100)"
            keyboardType="numeric"
          />
          <TouchableOpacity style={styles.button} onPress={runPrimes} disabled={loading}>
            <Text style={styles.buttonText}>Find Primes</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>⚡ Performance Benchmark</Text>
          <TouchableOpacity style={[styles.button, styles.benchmarkButton]} onPress={runBenchmark} disabled={loading}>
            <Text style={styles.buttonText}>Run Benchmark</Text>
          </TouchableOpacity>
        </View>

        {loading && (
          <View style={styles.loading}>
            <Text style={styles.loadingText}>Computing in Rust... ⚙️</Text>
          </View>
        )}

        <View style={styles.results}>
          <Text style={styles.resultsTitle}>📊 Results</Text>
          {results.map((result, index) => (
            <View key={index} style={styles.resultItem}>
              <View style={styles.resultHeader}>
                <Text style={styles.resultType}>{result.type}</Text>
                <Text style={styles.resultTime}>{result.time}ms</Text>
              </View>
              <Text style={styles.resultData} numberOfLines={3}>
                {JSON.stringify(result.data, null, 2)}
              </Text>
            </View>
          ))}
        </View>
      </ScrollView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  scrollView: {
    flex: 1,
  },
  header: {
    padding: 20,
    alignItems: 'center',
    backgroundColor: '#fff',
    marginBottom: 20,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#333',
  },
  subtitle: {
    fontSize: 16,
    color: '#666',
    marginTop: 5,
  },
  section: {
    backgroundColor: '#fff',
    margin: 10,
    padding: 15,
    borderRadius: 8,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: '600',
    marginBottom: 10,
    color: '#333',
  },
  input: {
    borderWidth: 1,
    borderColor: '#ddd',
    borderRadius: 6,
    padding: 12,
    marginBottom: 10,
    fontSize: 16,
  },
  button: {
    backgroundColor: '#007AFF',
    padding: 12,
    borderRadius: 6,
    alignItems: 'center',
  },
  benchmarkButton: {
    backgroundColor: '#FF6B35',
  },
  buttonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
  loading: {
    padding: 20,
    alignItems: 'center',
  },
  loadingText: {
    fontSize: 16,
    color: '#666',
  },
  results: {
    margin: 10,
    padding: 15,
    backgroundColor: '#fff',
    borderRadius: 8,
  },
  resultsTitle: {
    fontSize: 18,
    fontWeight: '600',
    marginBottom: 10,
    color: '#333',
  },
  resultItem: {
    marginBottom: 15,
    padding: 10,
    backgroundColor: '#f8f8f8',
    borderRadius: 6,
  },
  resultHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 5,
  },
  resultType: {
    fontSize: 16,
    fontWeight: '600',
    color: '#333',
  },
  resultTime: {
    fontSize: 14,
    color: '#007AFF',
    fontWeight: '500',
  },
  resultData: {
    fontSize: 12,
    color: '#666',
    fontFamily: 'monospace',
  },
});

export default App;
