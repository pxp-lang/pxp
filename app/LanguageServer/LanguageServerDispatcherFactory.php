<?php

namespace Pxp\LanguageServer;

use Pxp\LanguageServer\Handlers\CompletionHandler;
use Phpactor\LanguageServer\Adapter\Psr\AggregateEventDispatcher;
use Phpactor\LanguageServer\Core\Command\CommandDispatcher;
use Phpactor\LanguageServer\Core\Dispatcher\ArgumentResolver\ChainArgumentResolver;
use Phpactor\LanguageServer\Core\Dispatcher\ArgumentResolver\LanguageSeverProtocolParamsResolver;
use Phpactor\LanguageServer\Core\Dispatcher\ArgumentResolver\PassThroughArgumentResolver;
use Phpactor\LanguageServer\Core\Dispatcher\Dispatcher;
use Phpactor\LanguageServer\Core\Dispatcher\Dispatcher\MiddlewareDispatcher;
use Phpactor\LanguageServer\Core\Dispatcher\DispatcherFactory;
use Phpactor\LanguageServer\Core\Handler\HandlerMethodRunner;
use Phpactor\LanguageServer\Core\Handler\Handlers;
use Phpactor\LanguageServer\Core\Server\ClientApi;
use Phpactor\LanguageServer\Core\Server\ResponseWatcher\DeferredResponseWatcher;
use Phpactor\LanguageServer\Core\Server\RpcClient\JsonRpcClient;
use Phpactor\LanguageServer\Core\Server\Transmitter\MessageTransmitter;
use Phpactor\LanguageServer\Core\Service\ServiceManager;
use Phpactor\LanguageServer\Core\Service\ServiceProviders;
use Phpactor\LanguageServer\Core\Workspace\Workspace;
use Phpactor\LanguageServer\Handler\System\ServiceHandler;
use Phpactor\LanguageServer\Handler\TextDocument\TextDocumentHandler;
use Phpactor\LanguageServer\Handler\Workspace\CommandHandler;
use Phpactor\LanguageServer\Listener\ServiceListener;
use Phpactor\LanguageServer\Listener\WorkspaceListener;
use Phpactor\LanguageServer\Middleware\CancellationMiddleware;
use Phpactor\LanguageServer\Middleware\ErrorHandlingMiddleware;
use Phpactor\LanguageServer\Middleware\HandlerMiddleware;
use Phpactor\LanguageServer\Middleware\InitializeMiddleware;
use Phpactor\LanguageServer\Middleware\ResponseHandlingMiddleware;
use Phpactor\LanguageServer\Middleware\ShutdownMiddleware;
use Phpactor\LanguageServerProtocol\InitializeParams;
use Psr\EventDispatcher\EventDispatcherInterface;
use Psr\Log\LoggerInterface;

final class LanguageServerDispatcherFactory implements DispatcherFactory
{
    public function __construct(
        private LoggerInterface $logger
    ) {}

    public function create(MessageTransmitter $transmitter, InitializeParams $initializeParams): Dispatcher
    {
        $responseWatcher = new DeferredResponseWatcher();
        $clientApi = new ClientApi(new JsonRpcClient($transmitter, $responseWatcher));

        $serviceProviders = new ServiceProviders();
        $serviceManager = new ServiceManager($serviceProviders, $this->logger);

        $workspace = new Workspace($this->logger);

        $eventDispatcher = new AggregateEventDispatcher(
            new ServiceListener($serviceManager),
            new WorkspaceListener($workspace)
        );

        $handlers = new Handlers(
            new TextDocumentHandler($eventDispatcher),
            new ServiceHandler($serviceManager, $clientApi),
            new CommandHandler(new CommandDispatcher([])),
            new CompletionHandler,
        );

        $runner = new HandlerMethodRunner(
            $handlers,
            new ChainArgumentResolver(
                new LanguageSeverProtocolParamsResolver,
                new PassThroughArgumentResolver,
            )
        );

        return new MiddlewareDispatcher(
            new ErrorHandlingMiddleware($this->logger),
            new InitializeMiddleware($handlers, $eventDispatcher, [
                'name' => 'pxp',
                'version' => '0.0.2'
            ]),
            new ShutdownMiddleware($eventDispatcher),
            new ResponseHandlingMiddleware($responseWatcher),
            new CancellationMiddleware($runner),
            new HandlerMiddleware($runner)
        );
    }
}
