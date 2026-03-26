import { GraphQLError, GraphQLFormattedError } from 'graphql';

interface RestLikeError {
  success: false;
  error: {
    code: string;
    message: string;
    timestamp: string;
    path: string;
  };
}

export function formatGraphqlError(error: GraphQLError): GraphQLFormattedError {
  const statusCode = (error.extensions?.originalError as any)?.statusCode;
  const code =
    (error.extensions?.originalError as any)?.response?.error?.code ||
    error.extensions?.code ||
    (statusCode === 404 ? 'NOT_FOUND' : 'GEN_001');

  const payload: RestLikeError = {
    success: false,
    error: {
      code: String(code),
      message: error.message,
      timestamp: new Date().toISOString(),
      path: 'graphql',
    },
  };

  return {
    message: error.message,
    locations: error.locations,
    path: error.path,
    extensions: {
      ...error.extensions,
      response: payload,
    },
  };
}
