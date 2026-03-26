import { GraphQLError } from 'graphql';
import { formatGraphqlError } from '../graphql-error.formatter';

describe('formatGraphqlError', () => {
  it('adds REST-like error envelope under extensions.response', () => {
    const error = new GraphQLError('Boom', {
      extensions: {
        code: 'FORBIDDEN',
      },
    });

    const formatted = formatGraphqlError(error);
    expect(formatted.message).toBe('Boom');
    expect((formatted.extensions as any).response.success).toBe(false);
    expect((formatted.extensions as any).response.error.code).toBe('FORBIDDEN');
    expect((formatted.extensions as any).response.error.path).toBe('graphql');
  });
});
