/**
 * React hooks for PropChain SDK integration.
 *
 * These hooks provide idiomatic React patterns for connecting to
 * the blockchain, querying data, and subscribing to events.
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { client, isConnected, error } = usePropChain('ws://localhost:9944', {
 *     propertyRegistry: '5Grw...',
 *   });
 *
 *   const { property, loading } = useProperty(client, 1);
 *
 *   if (loading) return <Spinner />;
 *   return <div>{property?.metadata.location}</div>;
 * }
 * ```
 */

import { useState, useEffect, useCallback, useRef } from 'react';

// Types for hook return values (simplified for example app)

interface UsePropChainResult {
  client: unknown | null;
  isConnected: boolean;
  error: Error | null;
  disconnect: () => Promise<void>;
}

interface UsePropertyResult {
  property: Record<string, unknown> | null;
  loading: boolean;
  error: Error | null;
  refetch: () => void;
}

interface UsePortfolioResult {
  properties: Record<string, unknown>[];
  loading: boolean;
  error: Error | null;
  refetch: () => void;
}

/**
 * Hook for managing the PropChain client connection.
 *
 * @param wsEndpoint - WebSocket URL
 * @param addresses - Contract addresses
 * @returns Client instance, connection state, and disconnect function
 */
export function usePropChain(
  wsEndpoint: string,
  addresses: Record<string, string>,
): UsePropChainResult {
  const [client, setClient] = useState<unknown | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    let mounted = true;

    const connect = async () => {
      try {
        // In production:
        // const { PropChainClient } = await import('@propchain/sdk');
        // const c = await PropChainClient.create(wsEndpoint, addresses);
        // if (mounted) {
        //   setClient(c);
        //   setIsConnected(true);
        // }

        // For demo purposes:
        if (mounted) {
          setClient({ mock: true });
          setIsConnected(true);
        }
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err : new Error(String(err)));
        }
      }
    };

    connect();

    return () => {
      mounted = false;
    };
  }, [wsEndpoint]);

  const disconnect = useCallback(async () => {
    // In production: await (client as PropChainClient)?.disconnect();
    setClient(null);
    setIsConnected(false);
  }, [client]);

  return { client, isConnected, error, disconnect };
}

/**
 * Hook for querying a single property.
 *
 * @param client - PropChain client instance
 * @param propertyId - Property ID to query
 * @returns Property data, loading state, error, and refetch function
 */
export function useProperty(
  client: unknown | null,
  propertyId: number,
): UsePropertyResult {
  const [property, setProperty] = useState<Record<string, unknown> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const fetchCount = useRef(0);

  const fetch = useCallback(async () => {
    if (!client) return;

    setLoading(true);
    setError(null);

    try {
      // In production:
      // const result = await (client as PropChainClient).propertyRegistry.getProperty(propertyId);
      // setProperty(result);

      // Demo:
      setProperty({
        id: propertyId,
        owner: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        metadata: {
          location: 'Demo Property',
          size: 2500,
          valuation: '500000',
        },
      });
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
    } finally {
      setLoading(false);
    }
  }, [client, propertyId]);

  useEffect(() => {
    fetch();
  }, [fetch]);

  return { property, loading, error, refetch: fetch };
}

/**
 * Hook for querying an owner's portfolio.
 *
 * @param client - PropChain client instance
 * @param owner - Owner's address
 * @returns Portfolio data, loading state, error, and refetch function
 */
export function usePortfolio(
  client: unknown | null,
  owner: string,
): UsePortfolioResult {
  const [properties, setProperties] = useState<Record<string, unknown>[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetch = useCallback(async () => {
    if (!client || !owner) return;

    setLoading(true);
    setError(null);

    try {
      // In production:
      // const details = await (client as PropChainClient)
      //   .propertyRegistry.getPortfolioDetails(owner);
      // setProperties(details.properties);

      // Demo:
      setProperties([
        { id: 1, location: 'Demo Property 1', size: 2500, valuation: '500000' },
        { id: 2, location: 'Demo Property 2', size: 3500, valuation: '750000' },
      ]);
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
    } finally {
      setLoading(false);
    }
  }, [client, owner]);

  useEffect(() => {
    fetch();
  }, [fetch]);

  return { properties, loading, error, refetch: fetch };
}

/**
 * Hook for subscribing to contract events.
 *
 * @param client - PropChain client instance
 * @param eventName - Name of the event to listen for
 * @param callback - Called with each new event
 */
export function useContractEvents(
  client: unknown | null,
  eventName: string,
  callback: (event: unknown) => void,
): void {
  useEffect(() => {
    if (!client) return;

    // In production:
    // let subscription: Subscription;
    // const subscribe = async () => {
    //   subscription = await (client as PropChainClient)
    //     .propertyRegistry.on(eventName, callback);
    // };
    // subscribe();
    // return () => subscription?.unsubscribe();

    return undefined;
  }, [client, eventName, callback]);
}
