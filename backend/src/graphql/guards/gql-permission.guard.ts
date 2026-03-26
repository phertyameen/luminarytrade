import {
  Injectable,
  CanActivate,
  ExecutionContext,
  ForbiddenException,
} from '@nestjs/common';
import { Reflector } from '@nestjs/core';
import { GqlExecutionContext } from '@nestjs/graphql';
import { REQUIRE_PERMISSION } from '../../common/decorators/require-permission.decorator';
import { AuthorizationService } from '../../authorization/authorization.service';

@Injectable()
export class GqlPermissionGuard implements CanActivate {
  constructor(
    private readonly reflector: Reflector,
    private readonly authz: AuthorizationService,
  ) {}

  async canActivate(context: ExecutionContext): Promise<boolean> {
    const metadata = this.reflector.get(REQUIRE_PERMISSION, context.getHandler());
    if (!metadata) {
      return true;
    }

    const gqlContext = GqlExecutionContext.create(context);
    const request = gqlContext.getContext().req;
    const user = request.user;

    const allowed = await this.authz.hasPermission(
      user.id,
      metadata.resource,
      metadata.action,
      { user, resource: gqlContext.getArgs() },
    );

    if (!allowed) {
      throw new ForbiddenException();
    }

    return true;
  }
}
