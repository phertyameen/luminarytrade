import {
  IsArray,
  IsBoolean,
  IsInt,
  IsNotEmpty,
  IsNumber,
  IsObject,
  IsOptional,
  IsString,
  Max,
  Min,
  ValidateNested,
} from 'class-validator';
import { Type } from 'class-transformer';

export class AgentFilterInput {
  @IsOptional()
  @IsString()
  name?: string;

  @IsOptional()
  @IsArray()
  @IsString({ each: true })
  capabilities?: string[];

  @IsOptional()
  @IsInt()
  @Min(1)
  evolution_level_min?: number;

  @IsOptional()
  @IsInt()
  @Max(10)
  evolution_level_max?: number;
}

export class OracleFilterInput {
  @IsOptional()
  @IsString()
  pair?: string;

  @IsOptional()
  minTimestamp?: Date;

  @IsOptional()
  maxTimestamp?: Date;

  @IsOptional()
  @IsInt()
  @Min(1)
  @Max(100)
  limit?: number;
}

export class AuditLogFilterInput {
  @IsOptional()
  @IsString()
  wallet?: string;

  @IsOptional()
  @IsString()
  eventType?: string;

  @IsOptional()
  startDate?: Date;

  @IsOptional()
  endDate?: Date;

  @IsOptional()
  @IsInt()
  @Min(1)
  @Max(500)
  limit?: number;

  @IsOptional()
  @IsInt()
  @Min(0)
  offset?: number;
}

export class CreateAgentInput {
  @IsString()
  @IsNotEmpty()
  name: string;

  @IsOptional()
  @IsString()
  description?: string;

  @IsOptional()
  @IsObject()
  metadata?: Record<string, any>;

  @IsOptional()
  @IsArray()
  @IsString({ each: true })
  capabilities?: string[];

  @IsOptional()
  @IsInt()
  @Min(1)
  @Max(10)
  evolution_level?: number;
}

export class UpdateAgentInput {
  @IsOptional()
  @IsString()
  name?: string;

  @IsOptional()
  @IsString()
  description?: string;

  @IsOptional()
  @IsObject()
  metadata?: Record<string, any>;

  @IsOptional()
  @IsArray()
  @IsString({ each: true })
  capabilities?: string[];

  @IsOptional()
  @IsInt()
  @Min(1)
  @Max(10)
  evolution_level?: number;

  @IsOptional()
  @IsObject()
  performance_metrics?: Record<string, any>;

  @IsOptional()
  @IsBoolean()
  is_active?: boolean;
}

export class OracleFeedInput {
  @IsString()
  @IsNotEmpty()
  pair: string;

  @IsString()
  @IsNotEmpty()
  price: string;

  @IsInt()
  decimals: number;
}

export class UpdateOracleInput {
  @IsNumber()
  timestamp: number;

  @IsArray()
  @ValidateNested({ each: true })
  @Type(() => OracleFeedInput)
  feeds: OracleFeedInput[];

  @IsString()
  @IsNotEmpty()
  signature: string;

  @IsOptional()
  @IsString()
  signer?: string;
}

export class CreateSubmissionInput {
  @IsString()
  @IsNotEmpty()
  idempotencyKey: string;

  @IsObject()
  payload: Record<string, any>;
}
