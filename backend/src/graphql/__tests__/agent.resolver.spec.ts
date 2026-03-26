import { AgentResolver } from '../resolvers/agent.resolver';
import { GraphqlPubSub } from '../pubsub.service';

describe('AgentResolver', () => {
  const mockIndexerService = {
    findOne: jest.fn(),
    search: jest.fn(),
    create: jest.fn(),
    update: jest.fn(),
  };

  const pubSub = new GraphqlPubSub();
  const resolver = new AgentResolver(mockIndexerService as any, pubSub);

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('returns agent by id', async () => {
    mockIndexerService.findOne.mockResolvedValue({ id: 'a1' });
    await expect(resolver.agent('a1')).resolves.toEqual({ id: 'a1' });
  });

  it('maps pagination to page/limit', async () => {
    mockIndexerService.search.mockResolvedValue({ data: [{ id: 'a1' }] });
    const result = await resolver.agents(10, 20, { name: 'alice' } as any);

    expect(mockIndexerService.search).toHaveBeenCalledWith({
      name: 'alice',
      page: 3,
      limit: 10,
    });
    expect(result).toEqual([{ id: 'a1' }]);
  });

  it('publishes update events after mutation', async () => {
    const updated = { id: 'a1', name: 'updated' };
    mockIndexerService.update.mockResolvedValue(updated);
    const publishSpy = jest.spyOn(pubSub.pubSub, 'publish');

    await expect(resolver.updateAgent('a1', { name: 'updated' } as any)).resolves.toEqual(updated);
    expect(publishSpy).toHaveBeenCalled();
  });
});
