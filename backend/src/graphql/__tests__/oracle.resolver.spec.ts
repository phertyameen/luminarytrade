import { OracleResolver } from '../resolvers/oracle.resolver';
import { GraphqlPubSub } from '../pubsub.service';

describe('OracleResolver', () => {
  const mockOracleService = {
    getSnapshotAsOracle: jest.fn(),
    getLatest: jest.fn(),
    updateSnapshot: jest.fn(),
  };

  const pubSub = new GraphqlPubSub();
  const resolver = new OracleResolver(mockOracleService as any, pubSub);

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('returns oracle by snapshot id', async () => {
    mockOracleService.getSnapshotAsOracle.mockResolvedValue({ id: 's1' });
    await expect(resolver.oracle('s1')).resolves.toEqual({ id: 's1' });
  });

  it('filters latest oracle data by pair', async () => {
    mockOracleService.getLatest.mockResolvedValue([
      { pair: 'BTC/USD', price: '1', decimals: 8, timestamp: new Date(), snapshotId: 's1' },
      { pair: 'ETH/USD', price: '2', decimals: 8, timestamp: new Date(), snapshotId: 's2' },
    ]);

    const rows = await resolver.oracles({ pair: 'BTC/USD' } as any);
    expect(rows).toHaveLength(1);
    expect(rows[0].pair).toBe('BTC/USD');
  });
});
